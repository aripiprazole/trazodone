use std::fmt::{Debug, Result};
use std::fmt::{Display, Formatter};

pub use crate::ir::*;

pub struct Print<'a, P: Pretty + ?Sized>(usize, &'a P);

impl<'a, P: Pretty> Display for Print<'a, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.1.pretty(self.0, f)
    }
}

impl<'a, P: Pretty> Debug for Print<'a, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.1.pretty(self.0, f)
    }
}

pub trait Pretty {
    fn pretty(&self, indent: usize, f: &mut Formatter) -> Result;

    fn boxed(&self) -> Print<'_, Self>
    where
        Self: Sized,
    {
        Print(0, self)
    }

    fn indented(&self, indent: usize) -> Print<'_, Self>
    where
        Self: Sized,
    {
        Print(indent, self)
    }

    fn dump(&self)
    where
        Self: Sized,
    {
        println!("{}", Print(0, self));
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{{")?;
        for instruction in &self.instructions {
            instruction.pretty(2, f)?;
            writeln!(f)?;
        }
        write!(f, "}}")
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
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "#{}@{}", self.0, self.1)
    }
}

impl Pretty for IntValue {
    fn pretty(&self, indent: usize, f: &mut Formatter) -> Result {
        self.0.pretty(indent, f)
    }
}

impl Pretty for Color {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "&color {}", self.0)
    }
}

impl Pretty for Binary {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        write!(f, "({} {} {})", self.lhs.boxed(), self.op, self.rhs.boxed())
    }
}

impl Debug for PositionBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PositionBinary::Con(v) => write!(f, "{v}"),
            PositionBinary::Sum(a, b) => write!(f, "(+ {a:?} {b:?})"),
            PositionBinary::Sub(a, b) => write!(f, "(- {a:?} {b:?})"),
            PositionBinary::Mul(a, b) => write!(f, "(* {a:?} {b:?})"),
            PositionBinary::Div(a, b) => write!(f, "(/ {a:?} {b:?})"),
        }
    }
}

impl Pretty for Position {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        match self {
            Position::Named {
                reference_name,
                gate_index,
            } => {
                write!(f, "[@{reference_name} {gate_index:?}]")
            }
            Position::Host => write!(f, "[^host]"),
        }
    }
}

impl Pretty for Term {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        match self {
            Term::True => write!(f, "true"),
            Term::False => write!(f, "false"),
            Term::Current => write!(f, "^current-term"),
            Term::Tag(tag) => write!(f, "TAG:{tag:?}"),
            Term::Ext(id, tag) => write!(f, "EXT:{id:?}{tag:?}"),
            Term::Equal(lhs, rhs) => write!(f, "{} == {}", lhs.boxed(), rhs.boxed()),
            Term::LogicalOr(lhs, rhs) => write!(f, "{} || {}", lhs.boxed(), rhs.boxed()),
            Term::LogicalAnd(lhs, rhs) => write!(f, "{} && {}", lhs.boxed(), rhs.boxed()),
            Term::Ref(name) => write!(f, "@{name}"),
            Term::Create(value) => write!(f, "create {}", value.boxed()),
            Term::ArityOf(ArityOf { term }) => write!(f, "arity-of {}", term.boxed()),
            Term::GetNumber(GetNumber { term }) => write!(f, "get-number {}", term.boxed()),
            Term::GetExt(GetExt { term }) => write!(f, "get-ext {}", term.boxed()),
            Term::GetTag(GetTag { term }) => write!(f, "get-tag {}", term.boxed()),
            Term::Alloc(Alloc { size }) => write!(f, "alloc(arity: {size})"),
            Term::GetPosition(GetPosition { term, position }) => {
                write!(f, "get-position {} {position}", term.boxed())
            }
            Term::LoadArgument(LoadArgument { argument_index }) => {
                write!(f, "load-argument {argument_index}")
            }
            Term::TakeArgument(TakeArgument {
                position,
                argument_index,
            }) => write!(
                f,
                "take-argument {} {}",
                position.boxed(),
                argument_index.boxed()
            ),
            Term::NotFound(atom) => {
                write!(f, "{{! not-found {atom:?} !}}")
            }
        }
    }
}

impl Pretty for Value {
    fn pretty(&self, _indent: usize, f: &mut Formatter) -> Result {
        match self {
            Value::Erased => write!(f, "erased!"),
            Value::Lam(position) => write!(f, "(lam! {})", position.boxed()),
            Value::App(position) => write!(f, "(app! {})", position.boxed()),
            Value::U60(u60) => write!(f, "(u60! {})", u60.boxed()),
            Value::F60(f60) => write!(f, "(f60! {})", f60.boxed()),
            Value::Dp0(color, position) => {
                write!(f, "(dp0! {} {})", color.boxed(), position.boxed())
            }
            Value::Dp1(color, position) => {
                write!(f, "(dp1! {} {})", color.boxed(), position.boxed())
            }
            Value::Argument(position) => {
                write!(f, "(argument! {})", position.boxed())
            }
            Value::Atom(position) => {
                write!(f, "(atom! {})", position.boxed())
            }
            Value::Super(color, position) => {
                write!(f, "(super! {} {})", color.boxed(), position.boxed())
            }
            Value::Binary(binary, position) => {
                write!(f, "(binary! {} {})", binary.boxed(), position.boxed())
            }
            Value::Function(function_id, position) => {
                write!(
                    f,
                    "(function! {} {})",
                    function_id.boxed(),
                    position.boxed()
                )
            }
            Value::Constructor(function_id, position) => {
                write!(
                    f,
                    "(constructor! {} {})",
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
            Instruction::Let(Let { name, value }) => {
                write!(f, "{:>indent$}let {name} = {};", "", value.boxed())
            }
            Instruction::Return(term) => {
                write!(f, "{:>indent$}return {};", "", term.boxed())
            }
            Instruction::Link(Link { position, term }) => write!(
                f,
                "{:>indent$}link {} ({});",
                "",
                position.boxed(),
                term.boxed()
            ),
            Instruction::If(If {
                condition,
                then,
                otherwise,
            }) => {
                writeln!(f, "{:>indent$}if {} {{", "", condition.boxed())?;
                for instruction in &then.instructions {
                    instruction.pretty(indent + 2, f)?;
                    writeln!(f)?;
                }
                write!(f, "{:>indent$}}}", "")?;
                if let Some(otherwise) = otherwise {
                    writeln!(f, " else {{")?;
                    for instruction in &otherwise.instructions {
                        instruction.pretty(indent + 2, f)?;
                        writeln!(f)?;
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
        let instructions = Block::new(vec![
            Instruction::binding("arg_0", Term::load_arg(0)),
            Instruction::binding("arg_1", Term::load_arg(1)),
            Instruction::cond(
                Term::True,
                Block::new(vec![
                    Instruction::IncrementCost,
                    Instruction::binding("ctr_0", Term::get_position(Term::Current, 0)),
                    Instruction::link(Position::initial("ctr_0"), Term::Ref("arg_1".into())),
                    Instruction::link(Position::new("ctr_0", 1), Term::Ref("arg_0".into())),
                    Instruction::binding(
                        "done",
                        Term::create_constructor(
                            FunctionId::new("AGirl", 1),
                            Position::initial("ctr_0"),
                        ),
                    ),
                    Instruction::link(Position::Host, Term::reference("done")),
                    Instruction::ret(Term::False),
                ]),
                None,
            ),
            Instruction::ret(Term::False),
        ]);

        println!("{instructions:?}");
    }
}
