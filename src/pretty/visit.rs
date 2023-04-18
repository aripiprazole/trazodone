use std::fmt::{Display, Formatter};

use crate::ir::visit::{Instruction, Term};

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Insert(vlen, redex) => {
                write!(f, "insert! {vlen} {redex}")
            }
            Term::Redex => {
                write!(f, "redex!")
            }
            Term::CreateVBuf => {
                write!(f, "create_vbuf!")
            }
            Term::CheckVLen => {
                write!(f, "check_vlen!")
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
            Instruction::IncreaseLen(index) => write!(f, "increase_len! {index}"),
            Instruction::Visit(index) => write!(f, "visit! {index}"),
            Instruction::SetVBuf(term) => write!(f, "set_vbuf!({term}"),
            Instruction::SetGoup(term) => write!(f, "set_goup! {term}"),
            Instruction::SetVLen => write!(f, "set_vlen!"),
            Instruction::SetCont => write!(f, "set_cont!"),
            Instruction::SetHost => write!(f, "set_host!"),
        }
    }
}
