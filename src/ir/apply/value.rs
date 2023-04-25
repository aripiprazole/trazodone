use crate::ir::apply::{FunctionId, Position, Term};
use hvm::syntax::Oper;

#[derive(Debug, Clone)]
pub enum Value {
    /// Creates a [Tag::DUP0] value.
    Dp0(Color, Position),

    /// Creates a [Tag::DUP1] value.
    Dp1(Color, Position),

    /// Creates a [Tag::ARGUMENT] value.
    Argument(Position),

    /// Creates a [Tag::ATOM] value.
    Atom(Position),

    /// Creates a [Tag::LAM] value.
    Lam(Position),

    /// Creates a [Tag::APP] value.
    App(Position),

    /// Creates a [Tag::SUPER] value.
    Super(Color, Position),

    /// Creates a [Tag::BINARY] value.
    Binary(Binary, Position),

    /// Creates a [Tag::U60] value.
    U60(U60),

    /// Creates a [Tag::F60] value.
    F60(F60),

    /// Creates a [Tag::FUNCTION] value.
    Function(FunctionId, Position),

    /// Creates a [Tag::CONSTRUCTOR] value.
    Constructor(FunctionId, Position),

    /// Creates an [Tag::ERASED] value.
    Erased,
}

#[derive(Debug, Clone)]
pub struct Color(pub u64);

#[derive(Debug, Clone)]
pub struct U60(pub u64);

#[derive(Debug, Clone)]
pub struct F60(pub f64);

/// Represents a binary operation, with its operator and
/// its operands.
#[derive(Debug, Clone)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: Oper,
    pub rhs: Box<Term>,
}

impl Value {
    /// Creates a new U60 value.
    pub fn u60(u60: u64) -> Value {
        Value::U60(U60(u60))
    }

    /// Creates a new F60 value.
    pub fn f60(f60: f64) -> Value {
        Value::F60(F60(f60))
    }
}
