use std::fmt::Result;
use std::fmt::{Display, Formatter};

use crate::spec::*;

pub struct PrettyBox<'a, P: Pretty + ?Sized>(usize, &'a P);

impl<'a, P: Pretty> Display for PrettyBox<'a, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.1.pretty(self.0, f)
    }
}

pub trait Pretty {
    fn pretty(&self, indent: usize, f: &mut Formatter) -> Result;

    fn boxed(&self) -> PrettyBox<'_, Self>
    where
        Self: Sized,
    {
        PrettyBox(0, self)
    }

    fn indented(&self, indent: usize) -> PrettyBox<'_, Self>
    where
        Self: Sized,
    {
        PrettyBox(indent, self)
    }

    fn dump(&self)
    where
        Self: Sized,
    {
        println!("{}", PrettyBox(0, self));
    }
}

impl Pretty for U60 {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Pretty for F60 {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Pretty for FunctionId {
    fn pretty(&self, indent: usize, f: &mut Formatter) -> Result {
        self.0.pretty(indent, f)
    }
}

impl Pretty for IntValue {
    fn pretty(&self, indent: usize, f: &mut Formatter) -> Result {
        self.0.pretty(indent, f)
    }
}

impl Pretty for Position {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "[{} {}]", self.reference_name, self.gate_index)
    }
}

impl Pretty for Color {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "#color {}", self.0)
    }
}

impl Pretty for Binary {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "({} {} {})", self.lhs.boxed(), self.op, self.rhs.boxed())
    }
}

impl Pretty for Term {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        match self {
            Term::GetTag => write!(f, "(get-tag)"),
            Term::Ref(name) => write!(f, "($ {name})"),
            Term::Create(value) => write!(f, "(create {})", value.boxed()),
            Term::Alloc(Alloc { size }) => write!(f, "(alloc {size})"),
            Term::LoadArgument(LoadArgument {
                term,
                argument_index,
            }) => write!(f, "(load-argument {} {})", term.boxed(), argument_index),
            Term::TakeArgument(TakeArgument {
                position,
                argument_index,
            }) => write!(
                f,
                "(take-argument {} {})",
                position.boxed(),
                argument_index.boxed()
            ),
        }
    }
}

impl Pretty for Value {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        match self {
            Value::Erased => write!(f, "Erased!"),
            Value::Lam(position) => write!(f, "(Lam! {})", position.boxed()),
            Value::App(position) => write!(f, "(App! {})", position.boxed()),
            Value::U60(u60) => write!(f, "(U60! {})", u60.boxed()),
            Value::F60(f60) => write!(f, "(F60! {})", f60.boxed()),
            Value::Dp0(color, position) => {
                write!(f, "(Dp0! {} {})", color.boxed(), position.boxed())
            }
            Value::Dp1(color, position) => {
                write!(f, "(Dp1! {} {})", color.boxed(), position.boxed())
            }
            Value::Argument(position) => {
                write!(f, "(Argument! {})", position.boxed())
            }
            Value::Atom(position) => {
                write!(f, "(Atom! {})", position.boxed())
            }
            Value::Super(color, position) => {
                write!(f, "(Super! {} {})", color.boxed(), position.boxed())
            }
            Value::Binary(binary, position) => {
                write!(f, "(Binary! {} {})", binary.boxed(), position.boxed())
            }
            Value::Function(function_id, position) => {
                write!(
                    f,
                    "(Function! {} {})",
                    function_id.boxed(),
                    position.boxed()
                )
            }
            Value::Constructor(function_id, position) => {
                write!(
                    f,
                    "(Constructor! {} {})",
                    function_id.boxed(),
                    position.boxed()
                )
            }
        }
    }
}

impl Pretty for Instruction {
    fn pretty(&self, indent: usize, f: &mut Formatter) -> Result {
        match self {
            Instruction::Term(term) => write!(f, "{:>indent$}{};", "", term.boxed()),
            Instruction::IncrementCost => write!(f, "{:>indent$}increment-cost;", ""),
            Instruction::Collect(Collect { term }) => {
                write!(f, "{:>indent$}collect {};", "", term.boxed())
            }
            Instruction::Free(Free { position, arity }) => {
                write!(f, "{:>indent$}free {} {arity};", "", position.boxed())
            }
            Instruction::WHNF(WHNF { strictness_index }) => {
                write!(f, "{:>indent$}whnf {strictness_index};", "")
            }
            Instruction::Link(Link { position, term }) => write!(
                f,
                "{:>indent$}link {} {};",
                "",
                position.boxed(),
                term.boxed()
            ),
            Instruction::Let(Let { name, value }) => {
                write!(f, "{:>indent$}let ${name} = {};", "", value.boxed())
            }
            Instruction::If(If {
                condition,
                then,
                otherwise,
            }) => {
                write!(f, "{:>indent$}if {} {{", "", condition.boxed())?;
                for instruction in then {
                    instruction.pretty(indent + 2, f)?;
                }
                write!(f, "{:>indent$}}}", "")?;
                if let Some(otherwise) = otherwise {
                    write!(f, " else {{")?;
                    for instruction in otherwise {
                        instruction.pretty(indent + 2, f)?;
                    }
                    write!(f, "{:>indent$}}}", "")?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}