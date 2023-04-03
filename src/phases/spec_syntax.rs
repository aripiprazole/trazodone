use std::ops::Deref;

use crate::phases::{Result, Transform};
use crate::syntax;
use crate::syntax::*;

impl Transform for hvm::syntax::Rule {
    type Output = Rule;

    fn transform(self) -> Result<Self::Output> {
        let hvm::Term::Ctr { name, args } = *self.lhs else {
            return Err("rule is not a constructor".into());
        };
        let rhs = *self.rhs;

        Ok(Rule {
            name,
            parameters: specialize_parameters(args)?,
            value: rhs.transform()?,
        })
    }
}

impl Transform for hvm::syntax::Term {
    type Output = Term;

    fn transform(self) -> Result<Self::Output> {
        use hvm::syntax::Term::*;

        match self {
            U6O { numb } => Ok(Term::U60(numb)),
            F6O { numb } => Ok(Term::F60(numb)),
            Var { name } => Ok(Term::Atom(name)),
            Sup { box val0, box val1 } => Ok(Term::Super(Super {
                first: val0.transform()?.into(),
                second: val1.transform()?.into(),
            })),
            Lam { name, box body } => Ok(Term::Lam(syntax::Lam {
                parameter: name,
                value: body.transform()?.into(),
            })),
            App { box func, box argm } => Ok(Term::App(syntax::App {
                global_name: None,
                callee: func.transform()?.into(),
                arguments: vec![argm.transform()?],
            })),
            Ctr { name, args } => Ok(Term::App(syntax::App {
                global_name: Some(name.clone()),
                callee: Term::Atom(name).into(),
                arguments: args
                    .iter()
                    .map(Deref::deref)
                    .cloned()
                    .map(|term| term.transform())
                    .collect::<Result<_>>()?,
            })),
            Let {
                name,
                box expr,
                box body,
            } => Ok(Term::Let(syntax::Let {
                name,
                value: expr.transform()?.into(),
                body: body.transform()?.into(),
            })),
            Dup {
                nam0,
                nam1,
                box expr,
                box body,
            } => Ok(Term::Duplicate(Duplicate {
                from: nam0,
                to: nam1,
                value: expr.transform()?.into(),
                body: body.transform()?.into(),
            })),
            Op2 {
                oper,
                box val0,
                box val1,
            } => Ok(Term::Binary(Binary {
                lhs: val0.transform()?.into(),
                op: oper,
                rhs: val1.transform()?.into(),
            })),
        }
    }
}

impl Transform for hvm::rulebook::RuleBook {
    type Output = Vec<RuleGroup>;

    fn transform(self) -> Result<Self::Output> {
        self.rule_group
            .keys()
            .map(|name| RuleGroup::specialize(name.clone(), &self))
            .collect()
    }
}

impl RuleGroup {
    pub fn specialize(name: String, book: &hvm::rulebook::RuleBook) -> Result<Self> {
        let (_id, group) = book
            .rule_group
            .get(&name)
            .ok_or(format!("No such group: {name}"))?;
        let rules = group
            .iter()
            .map(|rule| rule.clone().transform())
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

#[allow(clippy::vec_box)]
fn specialize_parameters(parameters: Vec<Box<hvm::syntax::Term>>) -> Result<Vec<Parameter>> {
    use hvm::syntax::Term::*;

    parameters
        .iter()
        .map(Deref::deref)
        .cloned()
        .map(|term| match term {
            Var { name } if name == "*" => Ok(Parameter::Erased),
            Var { name } => Ok(Parameter::Atom(name)),
            Ctr { name, args } => Ok(Parameter::Constructor(Constructor {
                name,
                arity: args.len() as u64,
                flatten_patterns: specialize_flatten_patterns(args)?,
            })),
            U6O { numb } => Ok(Parameter::U60(numb)),
            _ => Err("Invalid pattern".into()),
        })
        .collect::<Result<_>>()
}

#[allow(clippy::vec_box)]
fn specialize_flatten_patterns(
    flatten_patterns: Vec<Box<hvm::syntax::Term>>,
) -> Result<Vec<Pattern>> {
    use hvm::syntax::Term::*;

    flatten_patterns
        .iter()
        .map(Deref::deref)
        .map(|term| match term {
            Var { name } if name == "*" => Ok(Pattern::Erased),
            Var { name } => Ok(Pattern::Atom(name.clone())),
            _ => Err("Invalid flatten pattern".into()),
        })
        .collect()
}
