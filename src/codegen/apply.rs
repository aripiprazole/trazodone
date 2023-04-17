use crate::codegen::GlobalContext;
use crate::ir::apply::*;
use crate::syntax;
use crate::syntax::Parameter;

pub type Insertion = Block;
pub type FreeIndex = u64;
pub type FreeArity = u64;
pub type FreeVec = Vec<(FreeIndex, FreeArity)>;

pub mod collect;
pub mod deconstruct;
pub mod free;
pub mod term;
pub mod variable;

pub struct Codegen {
    name_index: u64,
    variables: Vec<Term>,
    instructions: Block,
    global: Box<GlobalContext>,
}

impl Codegen {
    pub fn new(global: Box<GlobalContext>) -> Self {
        Self {
            global,
            name_index: 0,
            variables: Vec::new(),
            instructions: Block::default(),
        }
    }
}

impl syntax::RuleGroup {
    pub fn transform_with(self, context: Box<GlobalContext>) -> Result<RuleGroup> {
        Ok(RuleGroup {
            name: self.name.clone(),
            hvm_visit: Block::default(),
            hvm_apply: Codegen::new(context).build_apply(&self)?,
        })
    }
}

impl Codegen {
    pub fn build_apply(&mut self, group: &syntax::RuleGroup) -> Result<Block> {
        let rules = group.rules.clone();
        let strict_parameters = group.strict_parameters.clone();

        if rules.is_empty() {
            return Err("no rules".into());
        }

        for i in 0..strict_parameters.len() {
            self.instructions.push(Instruction::binding(
                &format!("arg{i}"),
                Term::load_arg(Term::Current, i as u64),
            ));
        }

        // TODO: superpose

        for rule in rules {
            let collect = self.create_collect(&rule);

            self.variables = collect.iter().map(|variable| variable.as_term()).collect();

            let mut match_rule = Term::True;
            for (i, parameter) in rule.parameters.iter().cloned().enumerate() {
                match_rule = Term::logical_and(match_rule, self.build_match(group, i, parameter));
                match_rule = match_rule.simplify().clone();
            }

            if match_rule.is_true() {
                self.instructions.push(Instruction::IncrementCost);
                let done = self.build_term(rule.value.clone());
                self.build_link(done);
                self.build_collect(collect);
                self.build_free(&rule, group);
                self.instructions.push(Instruction::Return(Term::True));
            } else {
                let mut then: Codegen = self.new_block(Instruction::IncrementCost);
                let constructor_parameters = rule
                    .parameters
                    .iter()
                    .enumerate()
                    .filter(|parameter| matches!(parameter.1, Parameter::Constructor(..)));
                for (index, parameter) in constructor_parameters {
                    let Parameter::Constructor(constructor) = parameter else {
                        continue;
                    };

                    let argument = Term::reference(&format!("arg{}", index));

                    for (sub, _) in constructor.flatten_patterns.iter().enumerate() {
                        let term = Term::load_arg(argument.clone(), sub as u64);
                        let inst = Instruction::binding(&format!("arg{}_{}", index, sub), term);
                        then.instructions.push(inst);
                    }
                }
                let done = then.build_term(rule.value.clone());
                then.build_link(done);
                then.build_collect(collect);
                then.build_free(&rule, group);
                then.instructions.push(Instruction::Return(Term::True));

                self.instructions
                    .push(Instruction::cond(match_rule, then.instructions, None));
            }
        }

        self.instructions.push(Instruction::Return(Term::False));

        Ok(self.instructions.clone())
    }

    pub fn build_link(&mut self, done: Term) {
        self.instructions
            .push(Instruction::link(Position::Host, done));
    }

    fn alloc(&mut self, size: u64) -> Term {
        // TODO:
        // This will avoid calls to alloc() by reusing nodes from the left-hand side. Sadly, this seems
        // to decrease HVM's performance in some cases, probably because of added cache misses. Perhaps
        // this should be turned off. I'll decide later.

        Term::alloc(size)
    }

    fn fresh_name(&mut self, name: &str) -> String {
        let name = format!("{name}_{}", self.name_index);
        self.name_index += 1;
        name
    }

    fn new_block(&self, instruction: Instruction) -> Self {
        Self {
            global: self.global.clone(),
            name_index: self.name_index,
            variables: self.variables.clone(),
            instructions: Block::with(instruction),
        }
    }
}
