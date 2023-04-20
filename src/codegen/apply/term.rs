use crate::codegen::apply::Codegen;
use crate::codegen::build_name;
use crate::ir::apply::*;
use crate::ir::syntax;

impl Codegen {
    pub fn build_term(&mut self, term: syntax::Term) -> Term {
        use crate::ir::syntax::Term::*;

        match term {
            U60(u60) => Term::create_u60(u60),
            F60(f60) => Term::create_f60(f60),
            Let(syntax::Let {
                box value,
                box body,
                ..
            }) => {
                let value = self.build_term(value);
                self.variables.push(value);
                let body = self.build_term(body);
                self.variables.pop();

                body
            }
            App(syntax::App {
                is_function: true,
                arguments,
                global_name: Some(global_name),
                ..
            }) => {
                let name = self.fresh_name("call");
                let value = self.alloc(arguments.len() as u64);
                self.instructions.push(Instruction::binding(&name, value));

                let compiled_global_name = build_name(&global_name);
                let arguments = arguments
                    .into_iter()
                    .map(|argument| self.build_term(argument))
                    .collect::<Vec<_>>();

                for (index, argument) in arguments.iter().enumerate() {
                    self.instructions.push(Instruction::link(
                        Position::new(&name, index as u64),
                        argument.clone(),
                    ))
                }

                let index = self
                    .global
                    .constructors
                    .get(&compiled_global_name)
                    .unwrap_or_else(|| panic!("no function for {}", compiled_global_name));

                Term::create_function(
                    FunctionId::new(&compiled_global_name, *index),
                    Position::initial(&name),
                )
            }
            App(syntax::App {
                is_function: false,
                arguments,
                global_name: Some(global_name),
                ..
            }) => {
                let name = self.fresh_name("constructor");

                let compiled_global_name = build_name(&global_name);
                let arguments = arguments
                    .into_iter()
                    .map(|argument| self.build_term(argument))
                    .collect::<Vec<_>>();

                let value = self.alloc(arguments.len() as u64);

                self.instructions.push(Instruction::binding(&name, value));

                for (index, argument) in arguments.iter().enumerate() {
                    self.instructions.push(Instruction::link(
                        Position::new(&name, index as u64),
                        argument.clone(),
                    ))
                }

                let index = self
                    .global
                    .constructors
                    .get(&compiled_global_name)
                    .unwrap_or_else(|| panic!("no constructor for {}", compiled_global_name));

                Term::create_constructor(
                    FunctionId::new(&compiled_global_name, *index),
                    Position::initial(&name),
                )
            }
            App(syntax::App {
                box callee,
                arguments,
                ..
            }) => {
                let name = self.fresh_name("app");
                let callee = self.build_term(callee);
                let argument = self.build_term(arguments.first().unwrap().clone());

                let done = self.alloc(2);
                self.instructions.push(Instruction::binding(&name, done));
                self.instructions
                    .push(Instruction::link(Position::initial(&name), callee));
                self.instructions
                    .push(Instruction::link(Position::new(&name, 1), argument));

                Term::create_app(Position::initial(&name))
            }
            Atom(syntax::Atom {
                name,
                index,
                field_index,
            }) => match self.variables.get(index as usize) {
                Some(value) => value.clone(),
                None => Term::NotFound(syntax::Atom {
                    name,
                    index,
                    field_index,
                }),
            },
            Duplicate(_) => todo!(),
            Lam(_) => todo!(),
            Super(_) => todo!(),
            Binary(_) => todo!(),
            Ref(_) => todo!(),
        }
    }
}
