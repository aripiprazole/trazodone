use std::error::Error;

use fxhash::FxHashMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;

pub mod agent;
pub mod bb;
pub mod functions;
pub mod instruction;
pub mod main;
pub mod position;
pub mod runtime;
pub mod term;
pub mod terminator;
pub mod value;

pub struct Codegen<'a> {
    pub context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,

    //>>>Contextual stuff
    /// The current function let bound names
    pub names: FxHashMap<String, BasicValueEnum<'a>>,

    /// The context parameter for the apply function
    pub ctx: Option<inkwell::values::BasicValueEnum<'a>>,

    /// The current basic block
    pub bb: Option<inkwell::basic_block::BasicBlock<'a>>,
    //<<<
}

macro_rules! erased_step {
    ($name:path) => {
        panic!("Erased step {}. Unreachable", stringify!($name))
    };
}

pub(crate) use erased_step;

impl<'a> Codegen<'a> {
    pub fn new(context: &'a Context) -> Result<Self, Box<dyn Error>> {
        let module = context.create_module("HVM");
        let codegen = Codegen {
            context,
            module,
            builder: context.create_builder(),

            names: FxHashMap::default(),
            ctx: None,
            bb: None,
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
        let mut codegen = Codegen::new(&context).unwrap();

        codegen_entry(&mut codegen, &book["Main"]);
        codegen_entry(&mut codegen, &book["Add"]);

        println!("{}", codegen.module.print_to_string().to_string_lossy());

        let _execution_engine = codegen
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("Could not create execution engine");
    }

    fn codegen_entry(codegen: &mut Codegen, fun: &RuleGroup) {
        let ir = fun.hvm_apply.clone().into_control_flow_graph();

        codegen.initialize_std_functions();
        codegen.build_apply_function(fun, ir);

        // Verify the LLVM module integrity
        codegen.module.verify().unwrap_or_else(|err| {
            println!("{}", codegen.module.print_to_string().to_string_lossy());
            panic!("Module is broken: {}", err.to_string_lossy());
        });
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
