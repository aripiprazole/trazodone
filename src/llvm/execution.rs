use std::ffi::CStr;

use llvm_sys::execution_engine::{
    LLVMCreateExecutionEngineForModule, LLVMGetFunctionAddress, LLVMOpaqueExecutionEngine,
};
use llvm_sys::prelude::LLVMModuleRef;

use crate::llvm::cstr::cstr;

pub struct ExecutionEngine(pub *mut LLVMOpaqueExecutionEngine);

impl ExecutionEngine {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn try_new(module: LLVMModuleRef) -> Result<Self, String> {
        unsafe {
            let mut ptr = std::mem::MaybeUninit::uninit();
            let mut err = std::mem::zeroed();

            if LLVMCreateExecutionEngineForModule(ptr.as_mut_ptr(), module, &mut err) != 0 {
                // In case of error, we must avoid using the uninitialized ExecutionEngineRef.
                assert!(!err.is_null());
                let err = CStr::from_ptr(err);
                return Err(format!("Failed to create execution engine: {:?}", err));
            }

            Ok(ExecutionEngine(ptr.assume_init()))
        }
    }

    pub fn get_function_address(&self, name: &str) -> u64 {
        unsafe { LLVMGetFunctionAddress(self.0, cstr!(name)) }
    }
}
