use crate::ir::graph::{BasicBlock, HasTerm};

pub type VisitBlock = BasicBlock<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    IncreaseLen(u64),
    Visit(u64),

    // *internal*
    SetVBuf(Term),
    SetGoup(Term),
    SetVLen,
    UpdateCont,
    UpdateHost,
}

#[derive(Debug, Clone)]
pub enum Term {
    Redex,

    // *internal*
    CreateVBuf,
    True,
    False,
    CheckVLen,
}

impl HasTerm for Instruction {
    type Term = Term;
}
