use crate::ir::*;
use crate::phases::Transform;
use crate::syntax;
use crate::syntax::Rule;

pub type Insertion = Block;
pub type FreeIndex = u64;
pub type FreeArity = u64;
pub type FreeVec = Vec<(FreeIndex, FreeArity)>;

#[derive(Default)]
pub struct Codegen {
    instructions: Block,
}

impl Transform for syntax::RuleGroup {
    type Output = RuleGroup;

    fn transform(self) -> Result<Self::Output> {
        Ok(RuleGroup {
            name: self.name.clone(),
            hvm_visit: Block::default(),
            hvm_apply: Codegen::default().build_apply(&self)?,
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
                &format!("arg_{i}"),
                Term::load_arg(i as u64),
            ));
        }

        // TODO: superpose

        for rule in rules {
            let mut match_rule = Term::True;

            for (i, parameter) in rule.parameters.iter().cloned().enumerate() {
                match_rule = Term::logical_and(match_rule, build_match(group, i, parameter));
                match_rule = match_rule.simplify().clone();
            }

            if match_rule.is_true() {
                let done = self.build_rule(&rule);
                self.build_link(done);
                self.build_collect(&rule);
                self.build_free(&rule, group);
            } else {
                let mut then: Codegen = Block::with(Instruction::IncrementCost).into();
                let done = then.build_rule(&rule);
                then.build_link(done);
                then.build_collect(&rule);
                then.build_free(&rule, group);

                self.instructions.push(Instruction::If(If {
                    condition: match_rule,
                    then: then.into(),
                    otherwise: None,
                }));
            }
        }

        Ok(self.instructions.clone())
    }

    pub fn build_rule(&mut self, _rule: &Rule) -> Term {
        // TODO
        Term::True
    }

    pub fn build_link(&mut self, done: Term) {
        self.instructions
            .push(Instruction::link(Position::Host, done));
    }

    pub fn build_collect(&mut self, rule: &Rule) {
        use syntax::Parameter::*;
        use syntax::Pattern;

        let erase = rule
            .parameters
            .iter()
            .enumerate()
            .flat_map(|(index, parameter)| match parameter {
                Erased => vec![Erase {
                    parameter: parameter.clone(),
                    index: index as u64,
                    field_index: None,
                }],
                Constructor(constructor) => constructor
                    .flatten_patterns
                    .iter()
                    .filter(|pattern| matches!(pattern, Pattern::Erased))
                    .enumerate()
                    .map(|(field_index, _)| Erase {
                        parameter: parameter.clone(),
                        index: index as u64,
                        field_index: Some(field_index as u64),
                    })
                    .collect::<Vec<_>>(),
                _ => vec![],
            })
            .collect::<Vec<Erase>>();

        for term in erase {
            let index = term.index;
            let argument = match term.field_index {
                Some(field_index) => Term::reference(&format!("arg_{index}_{field_index}")),
                None => Term::reference(&format!("arg_{index}")),
            };

            self.instructions.push(Instruction::collect(argument));
        }
    }

    pub fn build_free(&mut self, rule: &Rule, group: &syntax::RuleGroup) {
        use syntax::Parameter::*;

        let free = rule
            .parameters
            .iter()
            .enumerate()
            .flat_map(|(i, parameter)| match parameter {
                Constructor(constructor) => {
                    vec![(i as u64, constructor.arity)]
                }
                _ => vec![],
            })
            .collect::<Vec<_>>();

        let mut free = free
            .iter()
            .map(|(index, arity)| {
                let argument = Term::reference(&format!("arg_{index}"));

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

impl From<Codegen> for Block {
    fn from(codegen: Codegen) -> Self {
        codegen.instructions
    }
}

impl From<Block> for Codegen {
    fn from(instructions: Block) -> Self {
        Self { instructions }
    }
}

fn build_match(group: &syntax::RuleGroup, i: usize, parameter: syntax::Parameter) -> Term {
    use syntax::Parameter::*;

    let argument = Term::reference(&format!("arg_{i}"));

    match parameter {
        U60(value) => Term::logical_and(
            Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::U60)),
            Term::equal(Term::get_num(argument), Term::create_u60(value)),
        ),
        F60(value) => Term::logical_and(
            Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::F60)),
            Term::equal(Term::get_num(argument), Term::create_f60(value)),
        ),
        Constructor(syntax::Constructor { name, .. }) => Term::logical_and(
            Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::CONSTRUCTOR)),
            Term::equal(Term::get_num(argument), Term::ext(&name)),
        ),
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
