pub struct Variable<I> {
    pub declared_block: Box<Block<I>>,
    pub name: String,
}

pub enum Terminator<I> {
    Return(String),
    Branch(Box<Block<I>>, Box<Block<I>>),
    Jump(Box<Block<I>>),
}

pub struct Block<I> {
    pub variables: Vec<Variable<I>>,
    pub instructions: Vec<I>,
    pub terminator: Terminator<I>,
}
