use std::ops::Deref;

use hvm::rulebook::RuleBook;

use crate::tree::{
    App, Binary, Constructor, Duplicate, Lam, Let, Parameter, Pattern, Rule, Super, Term,
};

pub type Result<T> = std::result::Result<T, String>;

impl Rule {
    pub fn specialize(index: u64, book: RuleBook, rule: hvm::syntax::Rule) -> Result<Self> {
        let hvm::Term::Ctr { name, args } = rule.lhs;

        Ok(Self {
            name,
            strict: book.id_to_smap.contains_key(&index),
            patterns: Self::specialize_parameters(args)?,
            value: Self::specialize_term(*rule.rhs)?,
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
                arguments: Self::specialize_term(argm)?.into(),
            })),
            hvm::Term::Ctr { name, args } => Ok(Term::App(App {
                global_name: Some(name.clone()),
                callee: Term::Atom(name).into(),
                arguments: args.iter().map(Self::specialize_term).collect(),
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
            } => Term::Binary(Binary {
                lhs: Self::specialize_term(val0)?.into(),
                op: oper,
                rhs: Self::specialize_term(val1)?.into(),
            }),
        }
    }

    pub fn specialize_parameters(parameters: Vec<Box<hvm::syntax::Term>>) {
        parameters
            .iter()
            .map(Deref::deref)
            .map(|term| match term {
                hvm::Term::Var { name } => Parameter::Atom(name.clone()),
                hvm::Term::Ctr { name, args } => Parameter::Constructor(Constructor {
                    name: name.clone(),
                    arity: args.len() as u64,
                    flatten_patterns: Self::specialize_flatten_patterns(args)?,
                }),
                hvm::Term::U6O { numb } => Parameter::Const(numb.clone()),
                _ => Err("Invalid pattern".into()),
            })
            .collect()
    }

    fn specialize_flatten_patterns(flatten_patterns: &Vec<Box<hvm::Term>>) -> Vec<Pattern> {
        flatten_patterns
            .iter()
            .map(Deref::deref)
            .map(|term| match term {
                hvm::Term::Var { name } => Pattern::Atom(name.clone()),
                _ => Err("Invalid flatten pattern".into()),
            })
            .collect()
    }
}
