use std::error::Error;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

pub mod main;
pub mod bb;
pub mod runtime;

pub struct Codegen<'a> {
    pub context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
}

impl<'a> Codegen<'a> {
    pub fn new(context: &'a Context) -> Result<Self, Box<dyn Error>> {
        let module = context.create_module("HVM");
        let codegen = Codegen {
            context,
            module,
            builder: context.create_builder(),
        };

        Ok(codegen)
    }
}

#[cfg(test)]
mod tests {
    use fxhash::FxHashMap;
    use inkwell::OptimizationLevel;
    use crate::cli::eval::{ir_codegen_book, setup_global_context};
    use crate::ir::rule::RuleGroup;
    use super::*;

    #[test]
    pub fn it_works() {
        let book = setup_book();
        let context = Context::create();
        let codegen = Codegen::new(&context).unwrap();

        let add_fn = book.get("Add").unwrap();
        let add_fn_ir = add_fn.hvm_apply.clone().into_control_flow_graph();

        codegen.initialize_std_functions();
        codegen.build_apply_function(add_fn, add_fn_ir);

        let _execution_engine = codegen
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("Could not create execution engine");

        // Verify the LLVM module integrity
        codegen.module.verify().unwrap_or_else(|err| {
            println!("{}", codegen.module.print_to_string().to_string_lossy());
            panic!("Module is broken: {}", err.to_string_lossy());
        });

        println!("{}", codegen.module.print_to_string().to_string_lossy());
    }

    fn setup_book() -> FxHashMap<String, RuleGroup> {
        let code = std::fs::read_to_string("example.hvm").unwrap();
        let file = match hvm::language::syntax::read_file(&code) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to parse: {}", code);
                eprintln!("{}", err);
                panic!("Failed to parse")
            }
        };
        let book = hvm::language::rulebook::gen_rulebook(&file);

        let global = setup_global_context(&book);
        ir_codegen_book(&book, global)
    }
}
