use hvm::syntax::Oper;

use crate::ir::apply::{Arity, Binary, Color, FunctionId, Position, Tag, Value, F60, U60};

/// An internal intermediate representation construct that represents
/// a value.
#[derive(Debug, Clone)]
pub enum Term {
    /// Represents the current term in the reduce context.
    Current,

    /// Represents the id of a Tag, in a constant value.
    Tag(Tag),

    /// Gets the arity of the given term.
    ArityOf(ArityOf),

    TakeArgument(TakeArgument),
    LoadArgument(LoadArgument),

    /// Gets the extension of the given term.
    GetExt(GetExt),

    /// Gets the number of the given term.
    GetNumber(GetNumber),

    /// Gets the tag of the given term.
    GetTag(GetTag),

    /// Gets the position of the given term.
    GetPosition(GetPosition),

    /// Represents the creation of a HVM value, having the given [Value].
    Create(Value),

    /// Represents the allocation of a node in the HVM heap, having
    /// the given arity.
    Alloc(Alloc),

    /// Represents the building of a HVM agent, its
    /// composed by the [Term::alloc], that firsts allocates
    /// the memory for the agent, and then followed by a series of
    /// [Instruction::link], that link the arguments to the agent.
    ///
    /// Basically, this is a sugar syntax for better readability and debugging
    /// purposes.
    Agent(Agent),

    //>>> Internal
    /// Represents the id of an Extension, in a constant value.
    Ext(u64, String),

    //>>> Logical operations
    Equal(Box<Term>, Box<Term>),
    LogicalOr(Box<Term>, Box<Term>),
    LogicalAnd(Box<Term>, Box<Term>),
    //<<< Logical operations

    //>>> Values
    Ref(String),
    True,
    False,

    /// Represents a not found atom, that is used to debug
    /// purposes, it is not used in the HVM.
    /// If it's encountered in the codegen, or evaluation,
    /// it means that the compiler is not working properly.
    NotFound(crate::ir::syntax::Atom),
    //<<< Values
}

#[derive(Debug, Clone)]
pub struct LoadArgument {
    pub term: Box<Term>,
    pub argument_index: u64,
}

#[derive(Debug, Clone)]
pub struct TakeArgument {
    pub position: Position,
    pub argument_index: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Alloc {
    pub size: Arity,
}

#[derive(Debug, Clone)]
pub struct GetPosition {
    pub term: Box<Term>,
    pub position: u64,
}

#[derive(Debug, Clone)]
pub struct GetTag {
    pub term: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct GetNumber {
    pub term: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct GetExt {
    pub term: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct ArityOf {
    pub term: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub arity: Arity,
    pub arguments: Vec<Term>,
}

impl Term {
    pub fn get_tag(&self) -> Self {
        Term::GetTag(GetTag { term: self.clone().into() })
    }

    pub fn get_num(&self) -> Self {
        Term::GetNumber(GetNumber { term: self.clone().into() })
    }

    pub fn get_ext(&self) -> Self {
        Term::GetExt(GetExt { term: self.clone().into() })
    }

    pub fn arity_of(term: Term) -> Self {
        Term::ArityOf(ArityOf { term: term.into() })
    }

    pub fn get_position(term: Term, position: u64) -> Self {
        Term::GetPosition(GetPosition {
            term: term.into(),
            position,
        })
    }

    pub fn load_arg(term: Term, argument_index: u64) -> Self {
        Term::LoadArgument(LoadArgument {
            term: Box::new(term),
            argument_index,
        })
    }

    // internal
    pub fn reference(name: &str) -> Self {
        Term::Ref(name.into())
    }

    pub fn equal(lhs: Term, rhs: Term) -> Self {
        Term::Equal(lhs.into(), rhs.into())
    }

    pub fn logical_or(lhs: Term, rhs: Term) -> Self {
        Term::LogicalOr(lhs.into(), rhs.into())
    }

    pub fn logical_and(lhs: Term, rhs: Term) -> Self {
        Term::LogicalAnd(lhs.into(), rhs.into())
    }

    pub fn ext(id: u64, ext: &str) -> Self {
        Term::Ext(id, ext.into())
    }

    pub fn alloc(size: u64) -> Self {
        Term::Alloc(Alloc { size })
    }

    // create
    pub fn erased() -> Self {
        Term::Create(Value::Erased)
    }

    pub fn create_dp0(color: u64, position: Position) -> Self {
        Term::Create(Value::Dp0(Color(color), position))
    }

    pub fn create_dp1(color: u64, position: Position) -> Self {
        Term::Create(Value::Dp1(Color(color), position))
    }

    pub fn create_atom(position: Position) -> Self {
        Term::Create(Value::Atom(position))
    }

    pub fn create_argument(position: Position) -> Self {
        Term::Create(Value::Argument(position))
    }

    pub fn create_lam(position: Position) -> Self {
        Term::Create(Value::Lam(position))
    }

    pub fn create_app(position: Position) -> Self {
        Term::Create(Value::App(position))
    }

    pub fn create_super(color: u64, position: Position) -> Self {
        Term::Create(Value::Super(Color(color), position))
    }

    pub fn create_u60(u60: u64) -> Self {
        Term::Create(Value::U60(U60(u60)))
    }

    pub fn create_f60(f60: f64) -> Self {
        Term::Create(Value::F60(F60(f60)))
    }

    pub fn create_function(function_id: FunctionId, position: Position) -> Self {
        Term::Create(Value::Function(function_id, position))
    }

    pub fn create_constructor(function_id: FunctionId, position: Position) -> Self {
        Term::Create(Value::Constructor(function_id, position))
    }

    pub fn create_binary(lhs: Term, op: Oper, rhs: Term, position: Position) -> Self {
        Term::Create(Value::Binary(
            Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            },
            position,
        ))
    }

    // useful functions
    pub fn is_true(&self) -> bool {
        match self {
            Term::True => true,
            Term::LogicalAnd(left, right) => left.is_true() && right.is_true(),
            Term::LogicalOr(left, right) => left.is_true() || right.is_true(),
            _ => false,
        }
    }

    pub fn simplify(&self) -> &Self {
        match self {
            Term::LogicalAnd(left, right) => {
                if left.is_true() {
                    right.simplify()
                } else if right.is_true() {
                    left.simplify()
                } else {
                    self
                }
            }
            Term::LogicalOr(left, right) => {
                if left.is_true() {
                    left.simplify()
                } else if right.is_true() {
                    right.simplify()
                } else {
                    self
                }
            }
            otherwise => otherwise,
        }
    }
}
