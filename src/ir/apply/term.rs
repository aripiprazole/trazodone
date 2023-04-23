use hvm::syntax::Oper;
use crate::ir::apply::{Binary, Color, FunctionId, Position, Tag, Value, U60, F60};

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
    pub size: u64,
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
    pub arity: u64,
    pub arguments: Vec<Term>,
}

#[derive(Debug, Clone)]
pub enum Term {
    Current,

    Tag(Tag),
    ArityOf(ArityOf),
    GetExt(GetExt),
    GetNumber(GetNumber),
    GetTag(GetTag),
    GetPosition(GetPosition),
    Create(Value),
    TakeArgument(TakeArgument),
    LoadArgument(LoadArgument),
    Alloc(Alloc),
    Agent(Agent),

    // * Internal
    Ext(u64, String),
    Equal(Box<Term>, Box<Term>),
    LogicalOr(Box<Term>, Box<Term>),
    LogicalAnd(Box<Term>, Box<Term>),
    Ref(String),
    True,
    False,
    NotFound(crate::ir::syntax::Atom),
}

impl Term {
    pub fn get_tag(term: Term) -> Self {
        Term::GetTag(GetTag { term: term.into() })
    }

    pub fn get_num(term: Term) -> Self {
        Term::GetNumber(GetNumber { term: term.into() })
    }

    pub fn get_ext(term: Term) -> Self {
        Term::GetExt(GetExt { term: term.into() })
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
