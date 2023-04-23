use std::collections::HashMap;

use itertools::Itertools;

use crate::codegen::GlobalContext;
use crate::ir::apply::*;
use crate::ir::syntax;
use crate::ir::syntax::{Parameter, Rule};

pub type Insertion = Block;
pub type FreeIndex = u64;
pub type FreeArity = u64;
pub type FreeVec = Vec<(FreeIndex, FreeArity)>;

pub mod binary;
pub mod call;
pub mod collect;
pub mod deconstruct;
pub mod free;
pub mod metadata;
pub mod term;
pub mod variable;

pub type Result<T> = std::result::Result<T, String>;

pub struct Codegen {
    name_index: u64,
    variables: Vec<(String, Term)>,
    instructions: Block,
    lambdas: HashMap<u64, String>,
    global: Box<GlobalContext>,
    // constant section
    constant_extensions: HashMap<String, u64>,
    constant_tags: HashMap<String, u64>,
}

impl Codegen {
    pub fn new(global: Box<GlobalContext>) -> Self {
        Self {
            global,
            name_index: 0,
            lambdas: HashMap::new(),
            variables: Vec::new(),
            instructions: Block::default(),
            // constant sections
            constant_tags: HashMap::new(),
            constant_extensions: HashMap::new(),
        }
    }

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

            self.variables = collect
                .iter()
                .map(|variable| ("*".into(), variable.as_term()))
                .collect();

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
                self.instr(Instruction::Return(Term::True));
            } else {
                let mut then: Codegen = self.new_block(Instruction::IncrementCost);
                self.build_constructor_patterns(&rule, &mut then.instructions);
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

    pub fn tag(&mut self, tag: Tag) -> Term {
        self.constant_tags.insert(tag.to_string(), tag.id());

        Term::Tag(tag)
    }

    pub fn ext(&mut self, id: u64, name: &str) -> Term {
        self.constant_extensions.insert(name.into(), id);

        Term::ext(id, name)
    }

    pub fn build_link(&mut self, done: Term) {
        self.instructions
            .push(Instruction::link(Position::Host, done));
    }

    pub fn alloc_lam(&mut self, global_id: u64) -> String {
        if let Some(global_term) = self.lambdas.get(&global_id) {
            return global_term.clone();
        }

        let name = self.fresh_name("lam");
        self.instr(Instruction::binding(&name, Term::alloc(2)));

        if global_id != 0 {
            // FIXME: sanitizer still can't detect if a scope-less lambda doesn't use its bound
            //        variable, so we must write an Era() here. When it does, we can remove
            //        this line.
            self.instr(Instruction::link(Position::initial(&name), Term::erased()));
            self.lambdas.insert(global_id, name.clone());
        }

        name
    }

    pub fn alloc_tag(&mut self, tag: Tag) -> Term {
        self.constant_tags.insert(tag.to_string(), tag.id());

        Term::alloc(tag.size())
    }

    pub fn alloc(&mut self, size: u64) -> Term {
        // TODO:
        // This will avoid calls to alloc() by reusing nodes from the left-hand side. Sadly, this seems
        // to decrease HVM's performance in some cases, probably because of added cache misses. Perhaps
        // this should be turned off. I'll decide later.

        Term::alloc(size)
    }

    pub fn make_agent<F: FnOnce(&mut Vec<Term>)>(&mut self, f: F) -> Term {
        let mut arguments = Vec::new();
        f(&mut arguments);

        Term::Agent(Agent {
            arity: arguments.len() as u64,
            arguments: self.extract_argument_references(arguments),
        })
    }

    pub fn instr(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn get_name_id(&mut self, name: &str) -> u64 {
        let index = self
            .global
            .constructors
            .get(name)
            .unwrap_or_else(|| panic!("no constructor for {}", name));

        *index
    }

    fn build_constructor_patterns(&mut self, rule: &Rule, then: &mut Block) {
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
                then.block.push(inst);
            }
        }
    }

    fn fresh_name(&mut self, name: &str) -> String {
        let name = format!("{}", self.name_index);
        self.name_index += 1;
        name
    }

    fn new_block(&self, instruction: Instruction) -> Self {
        Self {
            global: self.global.clone(),
            name_index: self.name_index,
            variables: self.variables.clone(),
            lambdas: self.lambdas.clone(),
            instructions: Block::with(instruction),
            // constant clonning
            constant_extensions: self.constant_extensions.clone(),
            constant_tags: self.constant_tags.clone(),
        }
    }
    fn extract_argument_references(&mut self, arguments: Vec<Term>) -> Vec<Term> {
        arguments
            .iter()
            .cloned()
            // .map(|term| match term {
            //     term @ Term::Ref(..) => term,
            //     term => {
            //         let name = self.fresh_name("arg");
            //         self.instr(Instruction::binding(&name, term));
            //         Term::reference(&name)
            //     }
            // })
            .collect()
    }
}
