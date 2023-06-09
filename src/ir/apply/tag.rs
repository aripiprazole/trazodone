use std::fmt::Display;

use crate::ir::apply::{Arity, NameId};

/// Represents the builtin HVM value's tags.
#[derive(Debug, Clone)]
pub enum Tag {
    DUP0,        // Dp0
    DUP1,        // Dp1
    ATOM,        // Var
    ARGUMENT,    // Arg
    ERASED,      // Era
    LAM,         // Lam
    APP,         // App
    SUPER,       // Sup
    CONSTRUCTOR, // Ctr
    FUNCTION,    // Fun
    BINARY,      // Op2
    U60,         // U60
    F60,         // F60
    NIL,         // Nil
}

impl Tag {
    /// Returns the size of the tag in HVM arity.
    pub const fn size(&self) -> Arity {
        match self {
            Tag::DUP0 => 1,
            Tag::DUP1 => 1,
            Tag::ATOM => 1,
            Tag::ARGUMENT => 1,
            Tag::ERASED => 1,
            Tag::LAM => 2,
            Tag::SUPER => 3,
            Tag::APP => 2,
            Tag::CONSTRUCTOR => 2,
            Tag::FUNCTION => 2,
            Tag::BINARY => 2,
            Tag::U60 => 1,
            Tag::F60 => 1,
            Tag::NIL => 0,
        }
    }

    /// Returns the id of the tag.
    pub const fn id(&self) -> NameId {
        match self {
            Tag::DUP0 => 0x0,
            Tag::DUP1 => 0x1,
            Tag::ATOM => 0x2,
            Tag::ARGUMENT => 0x3,
            Tag::ERASED => 0x4,
            Tag::LAM => 0x5,
            Tag::APP => 0x6,
            Tag::SUPER => 0x7,
            Tag::CONSTRUCTOR => 0x8,
            Tag::FUNCTION => 0x9,
            Tag::BINARY => 0xa,
            Tag::U60 => 0xb,
            Tag::F60 => 0xc,
            Tag::NIL => 0xf,
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::DUP0 => {
                write!(f, "Dup0")
            }
            Tag::DUP1 => {
                write!(f, "Dup1")
            }
            Tag::ATOM => {
                write!(f, "Atom")
            }
            Tag::ARGUMENT => {
                write!(f, "Argument")
            }
            Tag::ERASED => {
                write!(f, "Erased")
            }
            Tag::LAM => {
                write!(f, "Lam")
            }
            Tag::APP => {
                write!(f, "App")
            }
            Tag::SUPER => {
                write!(f, "Super")
            }
            Tag::CONSTRUCTOR => {
                write!(f, "Constructor")
            }
            Tag::FUNCTION => {
                write!(f, "Function")
            }
            Tag::BINARY => {
                write!(f, "Binary")
            }
            Tag::U60 => {
                write!(f, "U60")
            }
            Tag::F60 => {
                write!(f, "F60")
            }
            Tag::NIL => {
                write!(f, "Nil")
            }
        }
    }
}
