use std::fmt::{Display, Formatter};

use crate::ir::syntax::{App, Atom, Binary, Duplicate, Lam, Let, Super, Term};

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::U60(value) => write!(f, "{}u60", value),
            Term::F60(value) => write!(f, "{}f60", value),
            Term::Let(Let {
                name,
                box value,
                box body,
            }) => write!(f, "let {name} = {value} in {body}"),
            Term::App(App {
                callee, arguments, ..
            }) if !arguments.is_empty() => {
                let arguments = arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                write!(f, "({callee} {arguments})",)
            }
            Term::App(App { callee, .. }) => {
                write!(f, "({callee})",)
            }
            Term::Lam(Lam {
                parameter,
                box value,
                ..
            }) => {
                write!(f, "λ{parameter} ({value})")
            }
            Term::Binary(Binary {
                op,
                box lhs,
                box rhs,
            }) => {
                write!(f, "({op} {lhs} {rhs})")
            }
            Term::Atom(Atom { name, .. }) => {
                write!(f, "{name}")
            }
            Term::Duplicate(Duplicate {
                from,
                to,
                box value,
                box body,
            }) => {
                write!(f, "dup {from} {to} = {value} in {body}")
            }
            Term::Super(Super { first, second }) => {
                write!(f, "({first}, {second})")
            }
        }
    }
}
