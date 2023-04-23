use std::fmt::{Display, Formatter};
use std::ops::Deref;

use hvm::syntax::Oper;

use crate::ir::syntax;

#[derive(Default, Clone)]
pub struct Block {
    pub tags: Vec<(String, u64)>,
    pub extensions: Vec<(String, u64)>,
    pub block: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Color(pub u64);

#[derive(Debug, Clone)]
pub struct U60(pub u64);

#[derive(Debug, Clone)]
pub struct F60(pub f64);

pub type DebugName = Option<String>;

#[derive(Debug, Clone)]
pub struct FunctionId(pub String, pub DebugName, pub u64);

#[derive(Debug, Clone)]
pub enum PositionBinary {
    Con(u64),
    Sum(Box<PositionBinary>, Box<PositionBinary>),
    Sub(Box<PositionBinary>, Box<PositionBinary>),
    Mul(Box<PositionBinary>, Box<PositionBinary>),
    Div(Box<PositionBinary>, Box<PositionBinary>),
}

#[derive(Debug, Clone)]
pub enum Position {
    Named {
        reference_name: String,
        gate_index: PositionBinary,
    },
    Host,
}

#[derive(Debug, Clone)]
pub struct IntValue(pub Value);

#[derive(Debug, Clone)]
pub struct LoadArgument {
    pub term: Box<Term>,
    pub argument_index: u64,
}

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

#[derive(Debug, Clone)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: Oper,
    pub rhs: Box<Term>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Dp0(Color, Position),
    Dp1(Color, Position),
    Argument(Position),
    Atom(Position),
    Lam(Position),
    App(Position),
    Super(Color, Position),
    Binary(Binary, Position),
    U60(U60),
    F60(F60),
    Function(FunctionId, Position),
    Constructor(FunctionId, Position),
    Erased,
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

    // * Internal
    Ext(u64, String),
    Equal(Box<Term>, Box<Term>),
    LogicalOr(Box<Term>, Box<Term>),
    LogicalAnd(Box<Term>, Box<Term>),
    Ref(String),
    True,
    False,
    NotFound(syntax::Atom),
}

#[derive(Debug, Clone)]
pub enum Tag {
    DUP0,
    DUP1,
    ATOM,
    ARGUMENT,
    ERASED,
    LAM,
    APP,
    SUPER,
    CONSTRUCTOR,
    FUNCTION,
    BINARY,
    U60,
    F60,
    NIL,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
}

#[derive(Debug)]
pub struct Variable {
    pub erased: bool,
    pub index: u64,
    pub field_index: Option<u64>,
}

impl Term {
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

    pub fn erased() -> Self {
        Term::Create(Value::Erased)
    }
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

impl Value {
    pub fn u60(u60: u64) -> Value {
        Value::U60(U60(u60))
    }

    pub fn f60(f60: f64) -> Value {
        Value::F60(F60(f60))
    }
}

impl Position {
    pub fn new(reference_name: &str, gate_index: u64) -> Self {
        Self::Named {
            reference_name: reference_name.into(),
            gate_index: PositionBinary::Con(gate_index),
        }
    }

    pub fn binary(reference_name: &str, gate_index: PositionBinary) -> Self {
        Self::Named {
            reference_name: reference_name.into(),
            gate_index,
        }
    }

    pub fn initial(reference_name: &str) -> Self {
        Self::new(reference_name, 0)
    }
}

impl FunctionId {
    pub fn new(name: &str, id: u64) -> Self {
        FunctionId(name.into(), None, id)
    }

    pub fn new_debug(name: &str, debug_name: String, id: u64) -> Self {
        FunctionId(name.into(), Some(debug_name), id)
    }
}

impl Deref for Block {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
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

impl Tag {
    pub const fn size(&self) -> u64 {
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
            Tag::NIL => 0
        }
    }

    pub const fn id(&self) -> u64 {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
