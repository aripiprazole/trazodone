use std::fmt::{Debug, Formatter};

pub trait HasTerm: Debug + Clone {
    type Term: Debug + Clone;
}

#[derive(Default, Clone)]
pub struct Label(pub String);

#[derive(Default, Debug, Clone)]
pub struct Variable {
    pub declared_block: Label,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Terminator<I: HasTerm> {
    Unreachable,
    Debug(String),
    Return(I::Term),
    Jump(Label),
    Cond(I::Term, Label, Label),
}

#[derive(Debug, Clone)]
pub struct BasicBlock<I: HasTerm> {
    pub label: String,
    pub variables: Vec<Variable>,
    pub instructions: Vec<I>,
    pub terminator: Terminator<I>,
    pub(crate) declared_blocks: Vec<BasicBlock<I>>,
}

impl Label {
    /// Creates a new label for the given basic block.
    ///  * label is a reference to the basic block.
    ///  * label is used to reference the basic block in the graph.
    pub fn new<I: HasTerm>(to: &BasicBlock<I>) -> Self {
        Self(to.label.clone())
    }
}

// Implemented manually to avoid the `BasicBlock` of being displayed recursively
impl Debug for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

// WONTFIX: The compiler throws an error if this is not implemented manually
// because the `I` type parameter is not used in the body of the
// function.
#[allow(clippy::derivable_impls)]
impl<I: HasTerm> Default for BasicBlock<I> {
    fn default() -> Self {
        Self {
            label: String::new(),
            variables: Vec::new(),
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
            // internal stuff
            declared_blocks: Vec::new(),
        }
    }
}

impl<I: HasTerm> BasicBlock<I> {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    pub fn with_return(&mut self, term: I::Term) {
        self.terminator = Terminator::Return(term);
    }

    pub fn with_debug(&mut self, message: String) {
        self.terminator = Terminator::Debug(message);
    }

    pub fn with_unreachable(&mut self) {
        self.terminator = Terminator::Unreachable;
    }

    pub fn with_cond(&mut self, cond: I::Term, then: &BasicBlock<I>, otherwise: &BasicBlock<I>) {
        self.terminator = Terminator::Cond(cond, Label::new(then), Label::new(otherwise));
    }
}
