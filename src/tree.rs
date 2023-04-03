use std::ops::Deref;

use hvm::syntax::Oper;

#[derive(Debug, Clone)]
pub struct RuleGroup {
    pub name: String,
    pub strict_index: u64,
    pub strict_parameters: Vec<bool>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Atom(String),
    Erased,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: String,
    pub arity: u64,
    pub flatten_patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
pub enum Parameter {
    Erased,
    Atom(String),
    U60(u64),
    F60(f64),
    Constructor(Constructor),
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub index: u64,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub value: Term,
}

#[derive(Debug, Clone)]
pub struct Duplicate {
    pub from: String,
    pub to: String,
    pub value: Box<Term>,
    pub body: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct App {
    pub global_name: Option<String>,
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: Box<Term>,
    pub body: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Super {
    pub first: Box<Term>,
    pub second: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Lam {
    pub parameter: String,
    pub value: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Ref {
    pub constructor: Box<Term>,
    pub meta_name: String,
    pub index: u64,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: Oper,
    pub rhs: Box<Term>,
}

#[derive(Debug, Clone)]
pub enum Term {
    U60(u64),
    F60(u64),

    Let(Let),
    App(App),
    Atom(String),
    Duplicate(Duplicate),
    Lam(Lam),
    Super(Super),
    Binary(Binary),

    // * Internals
    Ref(Ref),
}

pub type Result<T> = std::result::Result<T, String>;

impl RuleGroup {
    pub fn specialize(name: String, book: &hvm::rulebook::RuleBook) -> Result<Self> {
        let (_id, group) = book
            .rule_group
            .get(&name)
            .ok_or(format!("No such group: {name}"))?;
        let rules = group
            .iter()
            .enumerate()
            .map(|(index, rule)| Rule::specialize(index as u64, rule.clone()))
            .collect::<Result<Vec<_>>>()?;

        let strict_parameters = book
            .name_to_id
            .get(&name)
            .map(|id| book.id_to_smap.get(id).cloned().unwrap_or(vec![]))
            .unwrap_or(vec![]);

        let strict_index = strict_parameters.iter().filter(|x| **x).count() as u64;

        Ok(Self {
            name,
            strict_index,
            strict_parameters,
            rules,
        })
    }
}

impl Rule {
    pub fn specialize(index: u64, rule: hvm::syntax::Rule) -> Result<Self> {
        let hvm::Term::Ctr { name, args } = *rule.lhs else {
            return Err(format!("Rule {} is not a constructor", index));
        };
        let rhs = *rule.rhs;

        Ok(Self {
            index,
            name,
            parameters: Self::specialize_parameters(args)?,
            value: Self::specialize_term(rhs)?,
        })
    }

    pub fn specialize_term(term: hvm::syntax::Term) -> Result<Term> {
        match term {
            hvm::Term::U6O { numb } => Ok(Term::U60(numb)),
            hvm::Term::F6O { numb } => Ok(Term::F60(numb)),
            hvm::Term::Var { name } => Ok(Term::Atom(name)),
            hvm::Term::Sup { box val0, box val1 } => Ok(Term::Super(Super {
                first: Self::specialize_term(val0)?.into(),
                second: Self::specialize_term(val1)?.into(),
            })),
            hvm::Term::Lam { name, box body } => Ok(Term::Lam(Lam {
                parameter: name,
                value: Self::specialize_term(body)?.into(),
            })),
            hvm::Term::App { box func, box argm } => Ok(Term::App(App {
                global_name: None,
                callee: Self::specialize_term(func)?.into(),
                arguments: vec![Self::specialize_term(argm)?],
            })),
            hvm::Term::Ctr { name, args } => Ok(Term::App(App {
                global_name: Some(name.clone()),
                callee: Term::Atom(name).into(),
                arguments: args
                    .iter()
                    .map(Deref::deref)
                    .map(Clone::clone)
                    .map(Self::specialize_term)
                    .collect::<Result<_>>()?,
            })),
            hvm::Term::Let {
                name,
                box expr,
                box body,
            } => Ok(Term::Let(Let {
                name,
                value: Self::specialize_term(expr)?.into(),
                body: Self::specialize_term(body)?.into(),
            })),
            hvm::Term::Dup {
                nam0,
                nam1,
                box expr,
                box body,
            } => Ok(Term::Duplicate(Duplicate {
                from: nam0,
                to: nam1,
                value: Self::specialize_term(expr)?.into(),
                body: Self::specialize_term(body)?.into(),
            })),
            hvm::Term::Op2 {
                oper,
                box val0,
                box val1,
            } => Ok(Term::Binary(Binary {
                lhs: Self::specialize_term(val0)?.into(),
                op: oper,
                rhs: Self::specialize_term(val1)?.into(),
            })),
        }
    }

    pub fn specialize_parameters(
        parameters: Vec<Box<hvm::syntax::Term>>,
    ) -> Result<Vec<Parameter>> {
        parameters
            .iter()
            .map(Deref::deref)
            .map(|term| match term {
                hvm::Term::Var { name } if name == "*" => Ok(Parameter::Erased),
                hvm::Term::Var { name } => Ok(Parameter::Atom(name.clone())),
                hvm::Term::Ctr { name, args } => Ok(Parameter::Constructor(Constructor {
                    name: name.clone(),
                    arity: args.len() as u64,
                    flatten_patterns: Self::specialize_flatten_patterns(args)?,
                })),
                hvm::Term::U6O { numb } => Ok(Parameter::U60(numb.clone())),
                _ => Err("Invalid pattern".into()),
            })
            .collect::<Result<_>>()
    }

    fn specialize_flatten_patterns(flatten_patterns: &Vec<Box<hvm::Term>>) -> Result<Vec<Pattern>> {
        flatten_patterns
            .iter()
            .map(Deref::deref)
            .map(|term| match term {
                hvm::Term::Var { name } if name == "*" => Ok(Pattern::Erased),
                hvm::Term::Var { name } => Ok(Pattern::Atom(name.clone())),
                _ => Err("Invalid flatten pattern".into()),
            })
            .collect()
    }
}
