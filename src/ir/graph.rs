use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub trait HasTerm: Debug + Clone {
    type Term: Debug + Clone;
}

#[derive(Clone)]
pub struct Label<I: HasTerm> {
    pub basic_block: Rc<BasicBlock<I>>,
}

#[derive(Default, Debug, Clone)]
pub struct Variable<I: HasTerm> {
    pub declared_block: Rc<BasicBlock<I>>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Terminator<I: HasTerm> {
    Unreachable,
    Debug(String),
    Return(I::Term),
    Branch(Label<I>, Label<I>),
    Jump(Label<I>),
    Cond(I::Term, Label<I>, Label<I>),
}

#[derive(Debug, Clone)]
pub struct BasicBlock<I: HasTerm> {
    pub label: String,
    pub variables: Vec<Variable<I>>,
    pub instructions: Vec<I>,
    pub terminator: Terminator<I>,
}

impl<I: HasTerm> Label<I> {
    /// Creates a new label for the given basic block.
    ///  * label is a reference to the basic block.
    ///  * label is used to reference the basic block in the graph.
    pub fn new(to: Rc<BasicBlock<I>>) -> Self {
        Self { basic_block: to }
    }
}

// Implemented manually to avoid the `BasicBlock` of being displayed recursively
impl<I: HasTerm> Debug for Label<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.basic_block.label)
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
        }
    }
}

impl<I: HasTerm> BasicBlock<I> {
    pub fn new(label: &str) -> Rc<Self> {
        Rc::new(Self {
            label: label.into(),
            ..Default::default()
        })
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

    pub fn with_cond(&mut self, cond: I::Term, then: Rc<BasicBlock<I>>, otherwise: Rc<BasicBlock<I>>) {
        self.terminator = Terminator::Cond(cond, Label::new(then), Label::new(otherwise));
    }

    pub fn with_branch(&mut self, then: Rc<BasicBlock<I>>, otherwise: Rc<BasicBlock<I>>) {
        self.terminator = Terminator::Branch(Label::new(then), Label::new(otherwise));
    }
}
