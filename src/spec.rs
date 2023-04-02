use crate::tree::Binary;

pub enum Parameter {
    Atom(String),
    Erased,
}

pub type Block = Vec<Instruction>;

pub struct Col;

pub struct U60(pub u64);

pub struct F60(pub f64);

pub struct FunctionId(pub Value);

pub struct Position {
    pub reference_name: String,
    pub gate_index: u64,
}

pub struct IntValue(pub Value);

pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

pub struct LoadArgument {
    pub term: Value,
    pub argument_index: u64,
}

pub struct Link {
    pub position: Position,
    pub term: Value,
}

pub struct Collect {
    pub term: Term,
}

pub struct Free {
    pub position: Position,
    pub arity: u64,
}

pub struct Binding {
    pub name: String,
    pub value: Term,
}

pub struct If {
    pub condition: Term,
    pub then: Block,
    pub otherwise: Block,
}

pub struct WHNF {
    pub strictness_index: u64,
}

pub enum Instruction {
    If(If),
    Binding(Binding),
    Create(Value),
    Link(Link),
    Collect(Collect),
    Free(Free),
    WHNF(WHNF),
    IncrementCost,
}

pub enum Value {
    Dp0(Col, Position),
    Dp1(Col, Position),
    Argument(Position),
    Atom(Position),
    Lam(Position),
    App(Position),
    Super(Col, Position),
    Binary(Binary, Position),
    U60(U60),
    F60(F60),
    Function(FunctionId, Position),
    Constructor(FunctionId, Position),
    Erased,
}

pub struct TakeArgument {
    pub position: Position,
    pub argument_index: Term,
}

pub struct Alloc {
    pub size: u64,
}

pub enum Term {
    GetTag,
    Create(Value),
    TakeArgument(TakeArgument),
    LoadArgument(LoadArgument),
    Alloc(Alloc),
}

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
