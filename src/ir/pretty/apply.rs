use std::fmt::{Debug, Result};
use std::fmt::{Display, Formatter};

use crate::ir::apply::*;

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

macro_rules! indented {
    ($dst:expr, $n:expr, $($arg:tt)*) => {
        {
            write!($dst, "{:>n$}", "", n = $n)?;
            write!($dst, $($arg)*)
        }
    };
}

macro_rules! indentedln {
    ($dst:expr, $n:expr, $($arg:tt)*) => {
        {
            write!($dst, "{:>n$}", "", n = $n)?;
            writeln!($dst, $($arg)*)
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "tags:")?;
        for (name, id) in &self.tags {
            writeln!(f, "  {name}: u64 = {id:#01x}")?;
        }
        writeln!(f)?;
        writeln!(f, "extensions:")?;
        for (name, id) in &self.extensions {
            writeln!(f, "  {name}: u64 = {id:#01x} ; {id}")?;
        }
        writeln!(f)?;
        writeln!(f, "entry:")?;
        for instruction in &self.block {
            instruction.pretty(2, f)?;
        }
        Ok(())
    }
}

impl Display for U60 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Display for F60 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "'{}", self.1.clone().unwrap_or(self.0.clone()))
    }
}

impl Display for IntValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "&color {}", self.0)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} {} {})", self.op, self.lhs, self.rhs)
    }
}

impl Display for PositionBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PositionBinary::Con(v) => write!(f, "{v}"),
            PositionBinary::Sum(a, b) => write!(f, "(+ {a} {b})"),
            PositionBinary::Sub(a, b) => write!(f, "(- {a} {b})"),
            PositionBinary::Mul(a, b) => write!(f, "(* {a} {b})"),
            PositionBinary::Div(a, b) => write!(f, "(/ {a} {b})"),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Position::Named {
                reference_name,
                gate_index: PositionBinary::Con(0),
            } => {
                write!(f, "[%{reference_name}]")
            }
            Position::Named {
                reference_name,
                gate_index,
            } => {
                write!(f, "[%{reference_name} {gate_index}]")
            }
            Position::Host => write!(f, "[^host]"),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Term::True => write!(f, "true"),
            Term::False => write!(f, "false"),
            Term::Current => write!(f, "^current-term"),
            Term::Tag(tag) => write!(f, "'{tag}"),
            Term::Ext(_, tag) => write!(f, "'{tag}"),
            Term::Equal(lhs, rhs) => write!(f, "{lhs} == {rhs}"),
            Term::LogicalOr(lhs, rhs) => write!(f, "{lhs} || {rhs}"),
            Term::LogicalAnd(lhs, rhs) => write!(f, "{lhs} && {rhs}"),
            Term::Ref(name) => write!(f, "%{name}"),
            Term::Create(value) => write!(f, "{value}"),
            Term::ArityOf(ArityOf { term }) => write!(f, "{term}/arity"),
            Term::GetNumber(GetNumber { term }) => write!(f, "{term}/num"),
            Term::GetExt(GetExt { term }) => write!(f, "{term}/ext"),
            Term::GetTag(GetTag { term }) => write!(f, "{term}/tag"),
            Term::Alloc(Alloc { size }) => write!(f, "(alloc ~arity: {size})"),
            Term::NotFound(atom) => {
                write!(f, "(! not-found {atom:?} !)")
            }
            Term::GetPosition(GetPosition { term, position }) => {
                write!(f, "(get-position {term} {position})")
            }
            Term::LoadArgument(LoadArgument {
                term,
                argument_index,
            }) => {
                write!(f, "(load-argument {term} {argument_index})")
            }
            Term::TakeArgument(TakeArgument {
                position,
                argument_index,
            }) => write!(f, "(take-argument {position} {argument_index})",),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Value::Erased => write!(f, "Erased"),
            Value::Lam(position) => write!(f, "(Lam.new {position})"),
            Value::App(position) => write!(f, "(App.new {position})"),
            Value::U60(u60) => write!(f, "(U60.new {u60})"),
            Value::F60(f60) => write!(f, "(F60.new {f60})"),
            Value::Dp0(color, position) => {
                write!(f, "(Dp0.new {color} {position})")
            }
            Value::Dp1(color, position) => {
                write!(f, "(Dp1.new {color} {position})")
            }
            Value::Argument(position) => {
                write!(f, "(Argument.new {position})")
            }
            Value::Atom(position) => {
                write!(f, "(Atom.new {position})")
            }
            Value::Super(color, position) => {
                write!(f, "(Super.new {color} {position})")
            }
            Value::Binary(binary, position) => {
                write!(f, "(Binary.new {binary} {position})")
            }
            Value::Function(function_id, position) => {
                write!(f, "(Function.new {function_id} {position})")
            }
            Value::Constructor(function_id, position) => {
                write!(f, "(Constructor.new {function_id} {position})",)
            }
        }
    }
}

impl Pretty for Instruction {
    fn pretty(&self, n: usize, f: &mut Formatter) -> Result {
        match self {
            Instruction::Term(term) => indentedln!(f, n, "{term}"),
            Instruction::IncrementCost => indentedln!(f, n, "increment-cost"),
            Instruction::Collect(Collect { term }) => {
                indentedln!(f, n, "collect {term}")
            }
            Instruction::Free(Free { position, arity }) => {
                indentedln!(f, n, "free {position} {arity}")
            }
            Instruction::Let(Let { name, value }) => {
                indentedln!(f, n, "%{name} = {value}")
            }
            Instruction::Return(term) => {
                indentedln!(f, n, "ret {term}")
            }
            Instruction::Link(Link { position, term }) => {
                indentedln!(f, n, "link {position} {term}")
            }
            Instruction::If(If {
                condition,
                then,
                otherwise,
            }) => {
                indentedln!(f, n, "if {condition}:")?;
                for instruction in &then.block {
                    instruction.pretty(n + 2, f)?;
                }
                indented!(f, n, "")?;
                if let Some(otherwise) = otherwise {
                    writeln!(f, "else")?;
                    for instruction in &otherwise.block {
                        instruction.pretty(n + 2, f)?;
                    }
                }
                writeln!(f)
            }
            Instruction::Println(message) => {
                indentedln!(f, n, "println {message:?}")
            }
            Instruction::Metadata(metadata) => {
                for comment in &metadata.comments {
                    indentedln!(f, n, "; {comment}")?;
                }
                indentedln!(f, n, "; term = {term}", term = metadata.term)?;
                for instruction in &metadata.instructions {
                    instruction.pretty(n + 2, f)?;
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
        // let instructions = Block::new(vec![
        //     Instruction::binding("arg_0", Term::load_arg(0)),
        //     Instruction::binding("arg_1", Term::load_arg(1)),
        //     Instruction::cond(
        //         Term::True,
        //         Block::new(vec![
        //             Instruction::IncrementCost,
        //             Instruction::binding("ctr_0", Term::get_position(Term::Current, 0)),
        //             Instruction::link(Position::initial("ctr_0"), Term::Ref("arg_1".into())),
        //             Instruction::link(Position::new("ctr_0", 1), Term::Ref("arg_0".into())),
        //             Instruction::binding(
        //                 "done",
        //                 Term::create_constructor(
        //                     FunctionId::new("AGirl", 1),
        //                     Position::initial("ctr_0"),
        //                 ),
        //             ),
        //             Instruction::link(Position::Host, Term::reference("done")),
        //             Instruction::ret(Term::False),
        //         ]),
        //         None,
        //     ),
        //     Instruction::ret(Term::False),
        // ]);
        //
        // println!("{instructions:?}");
    }
}
