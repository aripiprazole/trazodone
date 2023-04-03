use hvm::syntax::Oper;

pub type Block = Vec<Instruction>;

#[derive(Debug)]
pub struct Color(pub u64);

#[derive(Debug)]
pub struct U60(pub u64);

#[derive(Debug)]
pub struct F60(pub f64);

#[derive(Debug)]
pub struct FunctionId(pub String);

#[derive(Debug)]
pub enum Position {
    Named {
        reference_name: String,
        gate_index: u64,
    },
    Host,
}

#[derive(Debug)]
pub struct IntValue(pub Value);

#[derive(Debug)]
pub struct LoadArgument {
    pub argument_index: u64,
}

#[derive(Debug)]
pub struct Link {
    pub position: Position,
    pub term: Term,
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
    Return(Term),
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
    Function(FunctionId, Position),
    Constructor(FunctionId, Position),
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
pub struct GetPosition {
    pub position: u64,
}

#[derive(Debug)]
pub enum Term {
    GetTag,
    GetPosition(GetPosition),
    Create(Value),
    TakeArgument(TakeArgument),
    LoadArgument(LoadArgument),
    Alloc(Alloc),

    // * Internal
    Ref(String),
    True,
    False,
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

impl Term {
    pub fn get_position(position: u64) -> Self {
        Term::GetPosition(GetPosition { position })
    }

    pub fn load_argument(argument_index: u64) -> Self {
        Term::LoadArgument(LoadArgument { argument_index })
    }

    pub fn reference(name: &str) -> Self {
        Term::Ref(name.into())
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
        Term::Create(Value::Binary(Binary { lhs: Box::new(lhs), op, rhs: Box::new(rhs) }, position))
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

    pub fn create_erased() -> Self {
        Term::Create(Value::Erased)
    }
}

impl Instruction {
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
            gate_index,
        }
    }

    pub fn initial(reference_name: &str) -> Self {
        Self::new(reference_name, 0)
    }
}

impl FunctionId {
    pub fn new(name: &str) -> Self {
        FunctionId(name.into())
    }
}
