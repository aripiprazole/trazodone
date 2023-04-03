use hvm::syntax::Oper;

pub type Block = Vec<Instruction>;

#[derive(Debug)]
pub struct Color(pub u64);

#[derive(Debug)]
pub struct U60(pub u64);

#[derive(Debug)]
pub struct F60(pub f64);

#[derive(Debug)]
pub struct FunctionId(pub Value);

#[derive(Debug)]
pub struct Position {
    pub reference_name: String,
    pub gate_index: u64,
}

#[derive(Debug)]
pub struct IntValue(pub Value);

#[derive(Debug)]
pub struct LoadArgument {
    pub term: Value,
    pub argument_index: u64,
}

#[derive(Debug)]
pub struct Link {
    pub position: Position,
    pub term: Value,
}

#[derive(Debug)]
pub struct Collect {
    pub term: Term,
}

#[derive(Debug)]
pub struct Free {
    pub position: Position,
    pub arity: u64,
}

#[derive(Debug)]
pub struct Let {
    pub name: String,
    pub value: Term,
}

#[derive(Debug)]
pub struct If {
    pub condition: Term,
    pub then: Block,
    pub otherwise: Option<Block>,
}

#[derive(Debug)]
pub struct WHNF {
    pub strictness_index: u64,
}

#[derive(Debug)]
pub enum Instruction {
    If(If),
    Let(Let),
    Link(Link),
    Collect(Collect),
    Free(Free),
    WHNF(WHNF),
    Term(Term),
    IncrementCost,
}

#[derive(Debug)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: Oper,
    pub rhs: Box<Term>,
}

#[derive(Debug)]
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
    Function(Box<FunctionId>, Position),
    Constructor(Box<FunctionId>, Position),
    Erased,
}

#[derive(Debug)]
pub struct TakeArgument {
    pub position: Position,
    pub argument_index: Box<Term>,
}

#[derive(Debug)]
pub struct Alloc {
    pub size: u64,
}

#[derive(Debug)]
pub enum Term {
    GetTag,
    Create(Value),
    TakeArgument(TakeArgument),
    LoadArgument(LoadArgument),
    Alloc(Alloc),

    // * Internal
    Ref(String),
}

#[derive(Debug)]
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

#[derive(Debug)]
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
