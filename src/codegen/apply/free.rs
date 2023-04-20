use crate::codegen::apply::{Codegen, FreeVec};
use crate::ir::apply::Term;
use crate::ir::apply::{Free, Instruction};
use crate::ir::syntax;
use crate::ir::syntax::Rule;

impl Codegen {
    pub fn create_free(&mut self, rule: &Rule) -> FreeVec {
        use crate::ir::syntax::Parameter::*;

        rule.parameters
            .iter()
            .enumerate()
            .flat_map(|(i, parameter)| match parameter {
                Constructor(constructor) => {
                    vec![(i as u64, constructor.arity)]
                }
                _ => vec![],
            })
            .collect::<Vec<_>>()
    }

    pub fn build_free(&mut self, rule: &Rule, group: &syntax::RuleGroup) {
        let mut free = self
            .create_free(rule)
            .iter()
            .map(|(index, arity)| {
                let argument = Term::reference(&format!("arg{index}"));

                (Term::get_position(argument, 0), *arity)
            })
            .collect::<Vec<_>>();

        free.push((
            Term::get_position(Term::Current, 0),
            group.strict_parameters.len() as u64,
        ));

        for must_free in free {
            self.instructions.push(Instruction::Free(Free {
                position: must_free.0,
                arity: must_free.1,
            }));
        }
    }
}
