use std::fmt::{Display, Formatter};

use crate::ir::visit::{Instruction, Term};

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Redex => {
                write!(f, "new_redex")
            }
            Term::CreateVBuf => {
                write!(f, "new_vbuf")
            }
            Term::CheckVLen => {
                write!(f, "vlen == 0")
            }
            Term::True => {
                write!(f, "true")
            }
            Term::False => {
                write!(f, "false")
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::IncreaseLen(index) => write!(f, "%vlen = + %vlen (int-is-whnf {index})"),
            Instruction::Visit(index) => write!(f, "(visit-argument {index})"),
            Instruction::SetVBuf(term) => write!(f, "%vbuf = {term}"),
            Instruction::SetGoup(term) => write!(f, "%go-up = {term}"),
            Instruction::SetVLen => write!(f, "%vlen = 0"),
            Instruction::UpdateCont => write!(f, "ctx.cont = %go-up"),
            Instruction::UpdateHost => write!(f, "ctx.host = $updated-host"),
        }
    }
}
