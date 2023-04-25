use std::ops::Deref;

use crate::codegen::apply::Codegen;
use crate::ir::apply::Term;

/// An argument to a function call. It can either be a value, or a constructor.
///
/// If it is a constructor, then it can hold multiple arguments.
#[derive(Debug, Clone)]
pub struct Argument(pub Term, pub Vec<Term>);

impl Codegen {
    pub fn get_argument(&mut self, i: usize) -> &mut Argument {
        self.arguments
            .get_mut(i)
            .unwrap_or_else(|| panic!("Argument {} not found", i))
    }
}

impl Argument {
    pub fn new(term: Term) -> Self {
        Self(term, vec![])
    }

    pub fn get_field(&self, i: usize) -> Term {
        self.1
            .get(i)
            .unwrap_or_else(|| panic!("Field {} not found", i))
            .clone()
    }

    pub fn add_field(&mut self, term: Term) {
        self.1.push(term);
    }

    pub fn set_field(&mut self, index: usize, term: Term) {
        self.1.insert(index, term);
    }

    pub fn unbox(&self) -> Term {
        self.0.clone()
    }
}

impl Deref for Argument {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
