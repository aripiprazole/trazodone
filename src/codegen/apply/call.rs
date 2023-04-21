use crate::codegen::apply::Codegen;
use crate::codegen::build_name;
use crate::ir::apply::{FunctionId, Instruction, Position, Term};
use crate::ir::syntax;

impl Codegen {
    pub fn build_app(&mut self, callee: syntax::Term, arguments: Vec<syntax::Term>) -> Term {
        let name = self.fresh_name("app");
        let callee = self.build_term(callee);
        let argument = self.build_term(arguments.first().unwrap().clone());

        let done = self.alloc(2);
        self.instr(Instruction::binding(&name, done));
        self.instr(Instruction::link(Position::initial(&name), callee));
        self.instr(Instruction::link(Position::new(&name, 1), argument));

        Term::create_app(Position::initial(&name))
    }

    pub fn build_constructor(&mut self, arguments: Vec<syntax::Term>, global_name: String) -> Term {
        let name = self.fresh_name("constructor");
        let value = self.alloc(arguments.len() as u64);
        self.instr(Instruction::binding(&name, value));

        let compiled_global_name = build_name(&global_name);

        self.build_arguments(&arguments, name.clone());

        let index = self
            .global
            .constructors
            .get(&compiled_global_name)
            .unwrap_or_else(|| panic!("no constructor for {}", compiled_global_name));

        Term::create_constructor(
            FunctionId::new_debug(&compiled_global_name, global_name, *index),
            Position::initial(&name),
        )
    }

    pub fn build_call(&mut self, arguments: Vec<syntax::Term>, global_name: String) -> Term {
        let name = self.fresh_name("call");
        let value = self.alloc(arguments.len() as u64);
        self.instr(Instruction::binding(&name, value));

        let compiled_global_name = build_name(&global_name);

        self.build_arguments(&arguments, name.clone());

        let index = self
            .global
            .constructors
            .get(&compiled_global_name)
            .unwrap_or_else(|| panic!("no function for {}", compiled_global_name));

        Term::create_function(
            FunctionId::new_debug(&compiled_global_name, global_name, *index),
            Position::initial(&name),
        )
    }

    fn build_arguments(&mut self, arguments: &[syntax::Term], name: String) {
        let arguments = arguments
            .iter()
            .map(|argument| self.build_term(argument.clone()))
            .collect::<Vec<_>>();

        for (index, argument) in arguments.iter().enumerate() {
            self.instructions.push(Instruction::link(
                Position::new(&name, index as u64),
                argument.clone(),
            ))
        }
    }
}
