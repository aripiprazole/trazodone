use crate::codegen::apply::Codegen;
use crate::codegen::build_name;
use crate::ir::apply::{FunctionId, Instruction, Position, Term};
use crate::ir::syntax;

impl Codegen {
    pub fn build_app(&mut self, callee: syntax::Term, arguments: Vec<syntax::Term>) -> Term {
        let name = self.fresh_name("app");
        let callee = self.build_term(callee);
        let argument = self.build_term(arguments.first().unwrap().clone());

        let done = self.make_agent(|arguments| {
            arguments.push(callee);
            arguments.push(argument);
        });
        self.instr(Instruction::binding(&name, done));

        Term::create_app(Position::initial(&name))
    }

    pub fn build_constructor(&mut self, arguments: Vec<syntax::Term>, global_name: String) -> Term {
        let arguments = arguments
            .iter()
            .map(|argument| self.build_term(argument.clone()))
            .collect::<Vec<_>>();

        let id = self.get_name_id(&global_name);
        let name = self.fresh_name("constructor");
        let value = self.make_agent(|builder| {
            builder.extend(arguments);
        });

        self.instr(Instruction::binding(&name, value));

        Term::create_constructor(FunctionId::new(&global_name, id), Position::initial(&name))
    }

    pub fn build_call(&mut self, arguments: Vec<syntax::Term>, global_name: String) -> Term {
        let arguments = arguments
            .iter()
            .map(|argument| self.build_term(argument.clone()))
            .collect::<Vec<_>>();

        let id = self.get_name_id(&global_name);
        let name = self.fresh_name("call");
        let value = self.make_agent(|builder| {
            builder.extend(arguments);
        });

        self.instr(Instruction::binding(&name, value));

        Term::create_function(FunctionId::new(&global_name, id), Position::initial(&name))
    }
}
