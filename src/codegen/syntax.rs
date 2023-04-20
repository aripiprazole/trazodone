use std::ops::Deref;

use hvm::rulebook::RuleBook;

use crate::ir::syntax;
use crate::ir::syntax::*;

pub type Result<T> = std::result::Result<T, String>;

pub trait Transform {
    type Output;

    fn transform(self) -> Result<Self::Output>;
}

pub struct Variable {
    pub name: Option<String>,
    pub index: u64,
    pub field_index: Option<u64>,
}

pub struct Context {
    pub index: u64,
    pub variables: Vec<Variable>,
    pub book: RuleBook,
}

trait ContextTransform {
    type Output;

    fn transform(self, context: &mut Context) -> Result<Self::Output>;
}

impl ContextTransform for hvm::syntax::Rule {
    type Output = Rule;

    fn transform(self, context: &mut Context) -> Result<Self::Output> {
        let hvm::Term::Ctr { name, args } = *self.lhs else {
            return Err("rule is not a constructor".into());
        };
        let rhs = *self.rhs;

        let mut context = Context {
            book: context.book.clone(),
            index: 0,
            variables: Vec::new(),
        };

        Ok(Rule {
            name,
            parameters: specialize_parameters(args, &mut context)?,
            value: rhs.transform(&mut context)?,
        })
    }
}

impl ContextTransform for hvm::syntax::Term {
    type Output = Term;

    fn transform(self, context: &mut Context) -> Result<Self::Output> {
        use hvm::syntax::Term::*;

        match self {
            U6O { numb } => Ok(Term::U60(numb)),
            F6O { numb } => Ok(Term::F60(numb as f64)),
            Var { name } => context
                .variables
                .iter()
                .enumerate()
                .find_map(|(index, variable)| {
                    if variable.name == Some(name.clone()) {
                        Some(Ok(Term::Atom(Atom {
                            name: name.clone(),
                            index: index as u64,
                            field_index: variable.field_index,
                        })))
                    } else {
                        None
                    }
                })
                .unwrap(),
            Sup { box val0, box val1 } => Ok(Term::Super(Super {
                first: val0.transform(context)?.into(),
                second: val1.transform(context)?.into(),
            })),
            Lam { name, box body } => {
                context.variables.push(Variable {
                    name: Some(name.clone()),
                    index: 0,
                    field_index: None,
                });

                Ok(Term::Lam(syntax::Lam {
                    parameter: name,
                    value: body.transform(context)?.into(),
                }))
            }
            App { box func, box argm } => Ok(Term::App(syntax::App {
                is_function: false,
                global_name: None,
                callee: func.transform(context)?.into(),
                arguments: vec![argm.transform(context)?],
            })),
            Ctr { name, args } => Ok(Term::App(syntax::App {
                is_function: context.book.ctr_is_fun.contains_key(&name),
                global_name: Some(name.clone()),
                callee: Term::Atom(Atom {
                    name,
                    index: 0,
                    field_index: None,
                })
                .into(),
                arguments: args
                    .iter()
                    .map(Deref::deref)
                    .cloned()
                    .map(|term| term.transform(context))
                    .collect::<Result<_>>()?,
            })),
            Let {
                name,
                box expr,
                box body,
            } => {
                context.variables.push(Variable {
                    name: Some(name.clone()),
                    index: 0,
                    field_index: None,
                });

                Ok(Term::Let(syntax::Let {
                    name,
                    value: expr.transform(context)?.into(),
                    body: body.transform(context)?.into(),
                }))
            }
            Dup {
                nam0,
                nam1,
                box expr,
                box body,
            } => Ok(Term::Duplicate(Duplicate {
                from: nam0,
                to: nam1,
                value: expr.transform(context)?.into(),
                body: body.transform(context)?.into(),
            })),
            Op2 {
                oper,
                box val0,
                box val1,
            } => Ok(Term::Binary(Binary {
                lhs: val0.transform(context)?.into(),
                op: oper,
                rhs: val1.transform(context)?.into(),
            })),
        }
    }
}

impl Transform for RuleBook {
    type Output = Vec<RuleGroup>;

    fn transform(self) -> Result<Self::Output> {
        self.rule_group
            .keys()
            .map(|name| RuleGroup::specialize(name.clone(), &self))
            .collect()
    }
}

impl RuleGroup {
    pub fn specialize(name: String, book: &RuleBook) -> Result<Self> {
        let (_id, group) = book
            .rule_group
            .get(&name)
            .ok_or(format!("No such group: {name}"))?;
        let rules = group
            .iter()
            .map(|rule| {
                rule.clone().transform(&mut Context {
                    book: book.clone(),
                    index: 0,
                    variables: Vec::new(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let strict_parameters = book
            .name_to_id
            .get(&name)
            .map(|id| book.id_to_smap.get(id).cloned().unwrap_or(vec![]))
            .unwrap_or(vec![]);

        let strict_index = strict_parameters
            .iter()
            .enumerate()
            .filter(|x| *x.1)
            .count() as u64;

        Ok(Self {
            name,
            strict_index,
            strict_parameters,
            rules,
        })
    }
}

#[allow(clippy::vec_box)]
fn specialize_parameters(
    parameters: Vec<Box<hvm::syntax::Term>>,
    context: &mut Context,
) -> Result<Vec<Parameter>> {
    use hvm::syntax::Term::*;

    parameters
        .iter()
        .map(Deref::deref)
        .cloned()
        .enumerate()
        .map(|(index, term)| match term {
            Var { name } if name == "*" => {
                context.variables.push(Variable {
                    name: None,
                    index: context.index,
                    field_index: None,
                });

                Ok(Parameter::Erased)
            }
            Var { name } => {
                context.variables.push(Variable {
                    name: Some(name.clone()),
                    index: context.index,
                    field_index: None,
                });

                Ok(Parameter::Atom(name))
            }
            Ctr { name, args } => Ok(Parameter::Constructor(Constructor {
                name,
                arity: args.len() as u64,
                flatten_patterns: specialize_flatten_patterns(args, index as u64, context)?,
            })),
            U6O { numb } => Ok(Parameter::U60(numb)),
            _ => Err("Invalid pattern".into()),
        })
        .collect::<Result<_>>()
}

#[allow(clippy::vec_box)]
fn specialize_flatten_patterns(
    flatten_patterns: Vec<Box<hvm::syntax::Term>>,
    index: u64,
    context: &mut Context,
) -> Result<Vec<Pattern>> {
    use hvm::syntax::Term::*;

    flatten_patterns
        .iter()
        .map(Deref::deref)
        .enumerate()
        .map(|(pattern_index, term)| match term {
            Var { name } if name == "*" => {
                context.variables.push(Variable {
                    name: None,
                    index,
                    field_index: Some(pattern_index as u64),
                });

                Ok(Pattern::Erased)
            }
            Var { name } => {
                context.variables.push(Variable {
                    name: Some(name.clone()),
                    index,
                    field_index: Some(pattern_index as u64),
                });

                Ok(Pattern::Atom(name.clone()))
            }
            _ => Err("Invalid flatten pattern".into()),
        })
        .collect()
}
