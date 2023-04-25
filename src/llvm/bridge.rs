use hvm::ReduceCtx;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::AddressSpace;
use llvm_sys::execution_engine::LLVMLinkInMCJIT;
use llvm_sys::target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget};

use crate::ir::rule::RuleGroup;

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
pub struct Bridge<'a> {
    pub context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
}

type EvalFn = fn(*mut RuleGroup, *mut ReduceCtx) -> bool;

impl<'a> Bridge<'a> {
    pub fn new(context: &'a Context) -> Self {
        let module = context.create_module("hvm__bridge");

        Self {
            context,
            module,
            builder: context.create_builder(),
        }
    }

    /// Create a function that can be called from llvm, and it can call a rust
    pub fn create(&self, f: EvalFn, name: &str, group: *mut RuleGroup) -> String {
        // Function signature: <<name>>(%ctx: *mut <<reduce_ctx>>) -> i1
        let function_type = self.context.bool_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()],
            false,
        );
        let function = self.module.add_function(name, function_type, None);

        // Function type: (*mut <<rule_group>>, *mut <<reduce_ctx>>) -> i1
        let bridge_function_type = self.context.bool_type().fn_type(
            &[
                self.context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into(),
                self.context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into(),
            ],
            false,
        );

        //>>>Create entry
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        //<<<

        //>>>Build rule group pointer with constant, to evaluate it
        let group = self.context.i64_type().const_int(group as u64, false);
        let group = self.builder.build_int_to_ptr(
            group,
            self.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .ptr_type(AddressSpace::default()),
            "rule_group_ptr",
        );
        //<<<

        //>>>Build function pointer within a non-closure function, to call it
        //   on llvm.
        let eval_fn = self.context.i64_type().const_int(f as usize as u64, false);
        let eval_fn = self.builder.build_int_to_ptr(
            eval_fn,
            bridge_function_type.ptr_type(AddressSpace::default()),
            "eval_fn",
        );
        //<<<

        //>>>Build bridge call
        let bridge_call = self
            .builder
            .build_indirect_call(
                bridge_function_type,
                eval_fn,
                &[group.into(), function.get_first_param().unwrap().into()],
                "bridge_call",
            )
            .try_as_basic_value()
            .left()
            .unwrap();
        //<<<

        //>>>Return bridge call value
        self.builder.build_return(Some(&bridge_call));
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
    use std::mem::transmute;
    use std::ptr::null_mut;

    use inkwell::OptimizationLevel;

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

            let context = Context::create();
            let bridge = Bridge::new(&context);
            let execution_engine = bridge
                .module
                .create_jit_execution_engine(OptimizationLevel::None)
                .unwrap();

            let group: &mut RuleGroup = Box::leak(Box::default());
            let name = bridge.create(eval_fn, &format!("__bridge__{}", group.name), group);

            println!("{}", bridge.module.print_to_string().to_string_lossy());

            let fun = execution_engine.get_function_address(&name).unwrap();
            let fun = transmute::<_, unsafe extern "C" fn(*mut ReduceCtx) -> bool>(fun);

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
                heap: transmute(null_mut::<u8>()),
                prog: transmute(null_mut::<u8>()),
                visit: transmute(null_mut::<u8>()),
                redex: transmute(null_mut::<u8>()),
            };

            println!("{}", fun(&mut ctx as *const _ as *mut _));
        };
    }
}
