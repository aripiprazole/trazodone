use hvm::ReduceCtx;
use llvm_sys::core::*;
use llvm_sys::execution_engine::LLVMLinkInMCJIT;
use llvm_sys::prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef};
use llvm_sys::target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget};

use crate::ir::rule::RuleGroup;
use crate::llvm::cstr::cstr;

/// FIXME: its throwing segfault or invalid free.
///
/// Bridge is a struct that contains the llvm module, context and builder.
///
/// It does bridge between rust and llvm. It creates a function that can be
/// called from llvm, and it can call a rust function, with the reference
/// to the rule group and the reduce context, passing the `group` as a
/// constant pointer, and the `ctx` as a function argument.
///
/// E.g:
/// ```rust
/// fn eval_fn(group: *mut RuleGroup, ctx: *mut ReduceCtx) -> bool {
///   // ... magic
/// }
/// ```
///
/// The `eval_fn` is a rust function, that can be called from llvm, and it
/// has the signature:
///
/// ```rust
/// type Fn = fn(*mut ReduceCtx) -> bool;
/// ```
///
/// Since, the `group` is a constant pointer, it can be passed as a constant
/// and dereferenced as hardcode, but the `ctx` is a function argument, so
/// this is passed as a function argument.
pub struct Bridge {
    pub module: LLVMModuleRef,
    pub context: LLVMContextRef,
    pub builder: LLVMBuilderRef,
}

type EvalFn = fn(*mut RuleGroup, *mut ReduceCtx) -> bool;

impl Bridge {
    pub unsafe fn new(name: &str) -> Self {
        let module = LLVMModuleCreateWithName(cstr!(name));
        let context = LLVMGetModuleContext(module);
        let builder = LLVMCreateBuilderInContext(context);

        Self {
            module,
            context,
            builder,
        }
    }

    pub unsafe fn create(&self, f: EvalFn, name: &str, group: *mut RuleGroup) -> String {
        let module = self.module;
        let builder = self.builder;

        // Function signature: <<name>>(%ctx: <<reduce_ctx>>) -> i1
        let mut parameters = [LLVMPointerType(LLVMInt8Type(), 0)];
        let function_type = LLVMFunctionType(LLVMInt1Type(), parameters.as_mut_ptr(), 1, 0);
        let apply_function = LLVMAddFunction(module, cstr!(name), function_type);

        // Function type: (*mut <<rule_group>>, *mut <<reduce_ctx>>) -> i1
        let mut bridge_function_parameters = [
            LLVMPointerType(LLVMInt8Type(), 0),
            LLVMPointerType(LLVMInt8Type(), 0),
        ];
        let bridge_function_type = LLVMFunctionType(
            LLVMInt1Type(),
            bridge_function_parameters.as_mut_ptr(),
            2,
            0,
        );

        //>>>Create entry
        let entry = LLVMAppendBasicBlock(apply_function, cstr!("entry"));
        LLVMPositionBuilderAtEnd(builder, entry);
        //<<<

        //>>>Build rule group pointer with constant, to evaluate it
        let rule_group_ptr = LLVMConstInt(LLVMInt64Type(), group as u64, 0);
        let rule_group_ptr = LLVMBuildIntToPtr(
            builder,
            rule_group_ptr,
            LLVMPointerType(LLVMPointerType(LLVMInt8Type(), 0), 0),
            cstr!("rule_group_ptr"),
        );
        //<<<

        //>>>Build function pointer within a non-closure function, to call it
        //   on llvm.
        let eval_fn_ptr = LLVMConstInt(LLVMInt64Type(), f as usize as u64, 0);
        let eval_fn_ptr = LLVMBuildIntToPtr(
            builder,
            eval_fn_ptr,
            LLVMPointerType(bridge_function_type, 0),
            cstr!("eval_fn"),
        );
        //<<<

        //>>>Build bridge call
        let mut eval_fn_ptr_arguments = [rule_group_ptr, LLVMGetParam(apply_function, 0)];
        let return_value = LLVMBuildCall2(
            builder,
            bridge_function_type,
            eval_fn_ptr,
            eval_fn_ptr_arguments.as_mut_ptr(),
            2,
            cstr!("bridge_call"),
        );
        //<<<

        //>>>Return bridge call value
        LLVMBuildRet(builder, return_value);
        //<<<

        name.into()
    }
}

pub unsafe fn initialize_llvm() {
    LLVMLinkInMCJIT();
    LLVM_InitializeNativeTarget();
    LLVM_InitializeNativeAsmPrinter();
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use llvm_sys::core::LLVMPrintModuleToString;

    use crate::llvm::execution::ExecutionEngine;

    use super::*;

    fn eval_fn(group: *mut RuleGroup, ctx: *mut ReduceCtx) -> bool {
        unsafe {
            println!("Bridged fn: (group) {:?}", group.read().name);
            println!("Bridged fn: (ctx) {:?}", ctx.read().hold);

            true
        }
    }

    #[test]
    fn it_works() {
        unsafe {
            initialize_llvm();

            let bridge = Bridge::new("hvm_apply");
            let group = RuleGroup::default();
            let group = Box::leak(Box::new(group));
            let name = format!("__bridge__{}", group.name);
            bridge.create(eval_fn, &name, group);

            let execution_engine = ExecutionEngine::try_new(bridge.module).unwrap();
            let module_string = LLVMPrintModuleToString(bridge.module);
            let module_string = CStr::from_ptr(module_string).to_string_lossy();

            println!("{module_string}");

            let llvm_fn = execution_engine.get_function_address(&name);
            let llvm_fn = std::mem::transmute::<_, extern "C" fn(*mut ReduceCtx) -> bool>(llvm_fn);

            // If we use the following code, the program will crash.
            // because, `&*` will deref the pointer, and then, the pointer will be used as a reference.
            // and my pointers are null references.
            #[allow(clippy::transmute_ptr_to_ref)]
            let mut ctx = ReduceCtx {
                tid: 0,
                term: 0,
                cont: &mut 0,
                host: &mut 0,
                hold: false,
                heap: std::mem::transmute(std::ptr::null_mut::<u8>()),
                prog: std::mem::transmute(std::ptr::null_mut::<u8>()),
                visit: std::mem::transmute(std::ptr::null_mut::<u8>()),
                redex: std::mem::transmute(std::ptr::null_mut::<u8>()),
            };

            println!("{}", llvm_fn(&mut ctx as *const _ as *mut _));
        };
    }
}
