use std::fmt::{Display, Formatter};

use crate::ir::graph::{BasicBlock, HasTerm, Terminator};

impl<I: HasTerm> Display for BasicBlock<I>
where
    I: Display,
    I::Term: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for declared_block in self.declared_blocks.iter() {
            writeln!(f, "{declared_block}")?;
        }

        writeln!(f, "{}:", self.label)?;

        for variable in self.variables.iter() {
            writeln!(f, "  using {}", variable.name)?;
        }

        for instruction in self.instructions.iter() {
            writeln!(f, "  {instruction}")?;
        }

        writeln!(f, "  {}", self.terminator)
    }
}

impl<I: HasTerm + Display> Display for Terminator<I>
where
    I::Term: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminator::Unreachable => write!(f, "unreachable"),
            Terminator::Debug(message) => write!(f, "dbg {message}"),
            Terminator::Return(value) => write!(f, "ret {value}"),
            Terminator::Jump(label) => write!(f, "jmp {label:?}"),
            Terminator::Cond(cond, then, otherwise) => {
                write!(f, "cond ({cond}) {then:?} {otherwise:?}")
            }
        }
    }
}
