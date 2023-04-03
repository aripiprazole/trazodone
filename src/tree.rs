use hvm::syntax::Oper;

#[derive(Debug)]
pub enum Pattern {
    Atom(String),
    Erased,
}

#[derive(Debug)]
pub struct Constructor {
    pub name: String,
    pub arity: u64,
    pub flatten_patterns: Vec<Pattern>,
}

#[derive(Debug)]
pub enum Parameter {
    Atom(String),
    Const(u64),
    Constructor(Constructor),
    Erased,
}

#[derive(Debug)]
pub struct Rule {
    pub strict: bool,
    pub name: String,
    pub patterns: Vec<Parameter>,
    pub value: Term,
}

#[derive(Debug)]
pub struct Duplicate {
    pub from: String,
    pub to: String,
    pub value: Box<Term>,
    pub body: Box<Term>,
}

#[derive(Debug)]
pub struct App {
    pub global_name: Option<String>,
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
}

#[derive(Debug)]
pub struct Let {
    pub name: String,
    pub value: Box<Term>,
    pub body: Box<Term>,
}

#[derive(Debug)]
pub struct Super {
    pub first: Box<Term>,
    pub second: Box<Term>,
}

#[derive(Debug)]
pub struct Lam {
    pub parameter: String,
    pub value: Box<Term>,
}

#[derive(Debug)]
pub struct Ref {
    pub constructor: Box<Term>,
    pub meta_name: String,
    pub index: u64,
}

#[derive(Debug)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: Oper,
    pub rhs: Box<Term>,
}

#[derive(Debug)]
pub enum Term {
    U60(u64),
    F60(u64),

    Let(Let),
    App(App),
    Atom(String),
    Duplicate(Duplicate),
    Lam(Lam),
    Super(Super),
    Binary(Binary),

    // * Internals
    Ref(Ref),
}
