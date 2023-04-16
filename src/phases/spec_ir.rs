use std::collections::HashMap;

use crate::ir::*;
use crate::syntax;
use crate::syntax::Rule;

pub type Insertion = Block;
pub type FreeIndex = u64;
pub type FreeArity = u64;
pub type FreeVec = Vec<(FreeIndex, FreeArity)>;

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

#[derive(Debug, Clone)]
pub struct GlobalContext {
    pub name_index: u64,
    pub constructors: HashMap<String, u64>,
}

impl Default for GlobalContext {
    fn default() -> Self {
        Self {
            name_index: 29, // precomp.rs
            constructors: HashMap::new(),
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
                Term::load_arg(i as u64),
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
                let done = then.build_term(rule.value.clone());
                then.build_link(done);
                then.build_collect(collect);
                then.build_free(&rule, group);
                then.instructions.push(Instruction::Return(Term::True));

                self.instructions
                    .push(Instruction::cond(match_rule, then.instructions, None));
            }
        }

        if self
            .instructions
            .last()
            .filter(|inst| matches!(inst, Instruction::Return(_)))
            .is_some()
        {
            self.instructions.push(Instruction::Return(Term::False));
        }

        Ok(self.instructions.clone())
    }

    pub fn build_term(&mut self, term: syntax::Term) -> Term {
        use syntax::Term::*;

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

    pub fn build_link(&mut self, done: Term) {
        self.instructions
            .push(Instruction::link(Position::Host, done));
    }

    pub fn create_collect(&mut self, rule: &Rule) -> Vec<Variable> {
        use syntax::Parameter::*;
        use syntax::Pattern;

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

    pub fn create_free(&mut self, rule: &Rule) -> FreeVec {
        use syntax::Parameter::*;

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
            .create_free(&rule)
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

    fn build_match(
        &self,
        group: &syntax::RuleGroup,
        i: usize,
        parameter: syntax::Parameter,
    ) -> Term {
        use syntax::Parameter::*;

        let argument = Term::reference(&format!("arg{i}"));

        match parameter {
            U60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::U60)),
                Term::equal(Term::get_num(argument), Term::create_u60(value)),
            ),
            F60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::F60)),
                Term::equal(Term::get_num(argument), Term::create_f60(value)),
            ),
            Constructor(syntax::Constructor { name, .. }) => {
                let compiled_global_name = build_name(&name);
                let id = self
                    .global
                    .constructors
                    .get(&compiled_global_name)
                    .unwrap_or_else(|| panic!("no constructor for {}", compiled_global_name));

                Term::logical_and(
                    Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::CONSTRUCTOR)),
                    Term::equal(Term::get_num(argument), Term::ext(*id, &name)),
                )
            }
            Atom(..) if group.strict_parameters[i] => {
                // TODO: hoas for kind2

                Term::logical_or(
                    Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::CONSTRUCTOR)),
                    Term::logical_or(
                        Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::U60)),
                        Term::equal(Term::get_tag(argument), Term::Tag(Tag::F60)),
                    ),
                )
            }
            _ => Term::True,
        }
    }
}

impl Variable {
    pub fn as_name(&self) -> String {
        match self.field_index {
            Some(field_index) => format!("arg{index}_{field_index}", index = self.index),
            None => format!("arg{index}", index = self.index),
        }
    }

    pub fn as_simple_name(&self) -> String {
        match self.field_index {
            Some(field_index) => format!("x{index}_{field_index}", index = self.index),
            None => format!("x{index}", index = self.index),
        }
    }

    pub fn as_term(&self) -> Term {
        Term::reference(&self.as_name())
    }
}

pub fn build_name(name: &str) -> String {
    // TODO: this can still cause some name collisions.
    // Note: avoiding the use of `$` because it is not an actually valid
    // identifier character in C.
    //let name = name.replace('_', "__");
    let name = name.replace('.', "_").replace('$', "_S_");
    format!("_{}_", name)
}
