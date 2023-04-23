use crate::ir::apply::{Block, Position, Term};

#[derive(Debug, Clone)]
pub struct Link {
    pub position: Position,
    pub term: Term,
}

#[derive(Debug, Clone)]
pub struct Collect {
    pub term: Term,
}

#[derive(Debug, Clone)]
pub struct Free {
    pub position: Term,
    pub arity: u64,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: Term,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Term,
    pub then: Block,
    pub otherwise: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub term: crate::ir::syntax::Term,
    pub comments: Vec<String>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    If(If),
    Let(Let),
    Link(Link),
    Collect(Collect),
    Metadata(Metadata),
    Free(Free),
    Term(Term),
    Return(Term),
    IncrementCost,
    //
    Println(String),
}

impl Instruction {
    pub fn println(name: &str) -> Self {
        Instruction::Println(name.into())
    }

    pub fn binding(name: &str, value: Term) -> Self {
        Instruction::Let(Let {
            name: name.into(),
            value,
        })
    }

    pub fn link(position: Position, term: Term) -> Self {
        Instruction::Link(Link { position, term })
    }

    pub fn cond(condition: Term, then: Block, otherwise: Option<Block>) -> Self {
        Instruction::If(If {
            condition,
            then,
            otherwise,
        })
    }

    pub fn ret(term: Term) -> Self {
        Instruction::Return(term)
    }

    pub fn collect(term: Term) -> Self {
        Instruction::Collect(Collect { term })
    }
}
