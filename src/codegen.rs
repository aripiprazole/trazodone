use std::ffi::CStr;
use std::fmt::Debug;
use llvm_sys::analysis::LLVMVerifierFailureAction::LLVMReturnStatusAction;
use llvm_sys::analysis::LLVMVerifyModule;
use llvm_sys::core::*;
use llvm_sys::error::LLVMDisposeErrorMessage;
use llvm_sys::prelude as llvm;
use crate::cstr::cstr;
use crate::syntax::Rule;

pub struct Codegen {
    pub context: llvm::LLVMContextRef,
    pub module: llvm::LLVMModuleRef,
    pub builder: llvm::LLVMBuilderRef,
    pub current_fn: llvm::LLVMValueRef,
    pub global_sym: llvm::LLVMValueRef,
}

impl Codegen {
    pub fn codegen_main(&mut self) -> llvm::LLVMValueRef {
        unsafe {
            let mut p = [];
            let t = LLVMFunctionType(LLVMInt32TypeInContext(self.context), p.as_mut_ptr(), 0, 0);
            let main = LLVMAddFunction(self.module, cstr!("main"), t);
            LLVMPositionBuilderAtEnd(
                self.builder,
                LLVMAppendBasicBlockInContext(self.context, main, cstr!("entry")),
            );

            main
        }
    }

    pub fn codegen_rulebook(&mut self, rule_book: hvm::language::rulebook::RuleBook) {
        for (_name, (_, group)) in rule_book.rule_group {
            for _rule in group {
            }
        }
    }

    pub fn codegen_rule(&mut self, _name: String, _rule: Rule) -> llvm::LLVMValueRef {
        unsafe {
            let mut p = [];
            let t = LLVMFunctionType(LLVMInt32TypeInContext(self.context), p.as_mut_ptr(), 0, 0);
            let main = LLVMAddFunction(self.module, cstr!("main"), t);
            LLVMPositionBuilderAtEnd(
                self.builder,
                LLVMAppendBasicBlockInContext(self.context, main, cstr!("entry")),
            );

            main
        }
    }

    pub fn verify_module(&self) -> Result<(), String> {
        unsafe {
            let mut err = std::mem::zeroed();

            if LLVMVerifyModule(self.module, LLVMReturnStatusAction, &mut err) == 1 {
                let message = CStr::from_ptr(err).to_string_lossy().to_string();
                LLVMDisposeErrorMessage(err);
                return Err(message);
            }

            Ok(())
        }
    }

    pub fn new() -> Self {
        unsafe {
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(cstr!("hvm"), context);
            let builder = LLVMCreateBuilderInContext(context);

            Self {
                context,
                module,
                builder,
                current_fn: std::ptr::null_mut(),
                global_sym: std::ptr::null_mut(),
            }
        }
    }
}

impl Debug for Codegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let s = LLVMPrintModuleToString(self.module);
            let s = CStr::from_ptr(s).to_string_lossy().to_string();
            write!(f, "{}", s)
        }
    }
}
