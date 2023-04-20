use crate::codegen::apply::Codegen;
use crate::ir::apply::{Instruction, Variable};
use crate::ir::syntax::Rule;

impl Codegen {
    pub fn create_collect(&mut self, rule: &Rule) -> Vec<Variable> {
        use crate::ir::syntax::Parameter::*;
        use crate::ir::syntax::Pattern;

        rule.parameters
            .iter()
            .enumerate()
            .flat_map(|(index, parameter)| match parameter {
                Erased => vec![Variable {
                    erased: true,
                    index: index as u64,
                    field_index: None,
                }],
                Atom(..) => vec![Variable {
                    erased: false,
                    index: index as u64,
                    field_index: None,
                }],
                Constructor(constructor) => constructor
                    .flatten_patterns
                    .iter()
                    .enumerate()
                    .map(|(field_index, pattern)| Variable {
                        erased: matches!(pattern, Pattern::Erased),
                        index: index as u64,
                        field_index: Some(field_index as u64),
                    })
                    .collect::<Vec<_>>(),
                _ => vec![],
            })
            .collect::<Vec<Variable>>()
    }

    pub fn build_collect(&mut self, collect: Vec<Variable>) {
        for term in collect {
            if term.erased {
                let argument = term.as_term();

                self.instructions.push(Instruction::collect(argument));
            }
        }
    }
}
