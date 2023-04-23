use std::ops::Deref;

pub use arity::*;
pub use function_id::*;
pub use id::*;
pub use instruction::*;
pub use op::*;
pub use position::*;
pub use tag::*;
pub use term::*;
pub use value::*;

pub mod arity;
pub mod function_id;
pub mod id;
pub mod instruction;
pub mod op;
pub mod position;
pub mod tag;
pub mod term;
pub mod value;

#[derive(Default, Clone)]
pub struct Block {
    pub tags: Vec<(String, u64)>,
    pub extensions: Vec<(String, u64)>,
    pub block: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Variable {
    pub erased: bool,
    pub index: u64,
    pub field_index: Option<u64>,
}

impl Block {
    pub fn with(instruction: Instruction) -> Self {
        Self {
            extensions: vec![],
            tags: vec![],
            block: vec![instruction],
        }
    }

    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            extensions: vec![],
            tags: vec![],
            block: instructions,
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.block.push(instruction);
    }
}

impl Deref for Block {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}
