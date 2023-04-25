use std::error::Error;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

pub struct Codegen<'a> {
    pub context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
    pub execution_engine: ExecutionEngine<'a>,
}

impl<'a> Codegen<'a> {
    pub fn new(context: &'a Context) -> Result<Self, Box<dyn Error>> {
        let module = context.create_module("sum");
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
        let codegen = Codegen {
            context,
            module,
            builder: context.create_builder(),
            execution_engine,
        };

        Ok(codegen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let context = Context::create();
        let codegen = Codegen::new(&context).unwrap();
    }
}
