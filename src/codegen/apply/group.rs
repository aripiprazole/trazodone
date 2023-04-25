use itertools::Itertools;
use crate::codegen::apply::argument::Argument;

use crate::codegen::apply::Codegen;
use crate::ir::apply::{Block, Instruction, Term};
use crate::ir::syntax;

impl Codegen {
    pub fn build_apply(&mut self, group: &syntax::RuleGroup) -> super::Result<Block> {
        let rules = group.rules.clone();
        let strict_parameters = group.strict_parameters.clone();

        if rules.is_empty() {
            return Err("no rules".into());
        }

        for i in 0..strict_parameters.len() {
            let name = self.fresh_name("arg");
            let term = Term::load_arg(Term::Current, i as u64);

            self.instr(Instruction::binding(&name, term));
            self.arguments.push(Argument::new(Term::reference(&name)));
        }

        // TODO: superpose

        for rule in rules {
            let collect = self.create_collect(&rule);

            let mut match_rule = Term::True;
            for (i, parameter) in rule.parameters.iter().cloned().enumerate() {
                match_rule = Term::logical_and(match_rule, self.build_match(group, i, parameter));
                match_rule = match_rule.simplify().clone();
            }

            if match_rule.is_true() {
                self.instructions.push(Instruction::IncrementCost);
                self.variables = collect
                    .iter()
                    .map(|variable| self.variable_as_tuple(variable))
                    .collect();
                let done = self.build_term(rule.value.clone());
                self.build_link(done);
                self.build_collect(collect);
                self.build_free(&rule, group);
                self.instr(Instruction::Return(Term::True));
            } else {
                let mut then: Codegen = self.new_block(Instruction::IncrementCost);
                then.build_constructor_patterns(&rule);
                then.variables = collect
                    .iter()
                    .map(|variable| then.variable_as_tuple(variable))
                    .collect();
                let done = then.build_term(rule.value.clone());
                then.build_link(done);
                then.build_collect(collect);
                then.build_free(&rule, group);
                then.instr(Instruction::Return(Term::True));

                self.instr(Instruction::cond(match_rule, then.instructions, None));
            }
        }

        self.instructions.push(Instruction::Return(Term::False));

        self.instructions.tags = self
            .constant_tags
            .iter()
            .sorted_by_key(|(_, id)| **id)
            .map(|(name, id)| (name.clone(), *id))
            .collect();

        self.instructions.extensions = self
            .constant_extensions
            .iter()
            .sorted_by_key(|(_, id)| **id)
            .map(|(name, id)| (name.clone(), *id))
            .collect();

        Ok(self.instructions.clone())
    }
}
