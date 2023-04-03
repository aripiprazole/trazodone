use hvm::syntax::Oper;

#[derive(Debug, Clone)]
pub struct RuleGroup {
    pub name: String,
    pub strict_index: u64,
    pub strict_parameters: Vec<bool>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Atom(String),
    Erased,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: String,
    pub arity: u64,
    pub flatten_patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
pub enum Parameter {
    Erased,
    Atom(String),
    U60(u64),
    F60(f64),
    Constructor(Constructor),
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub value: Term,
}

#[derive(Debug, Clone)]
pub struct Duplicate {
    pub from: String,
    pub to: String,
    pub value: Box<Term>,
    pub body: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct App {
    pub global_name: Option<String>,
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: Box<Term>,
    pub body: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Super {
    pub first: Box<Term>,
    pub second: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Lam {
    pub parameter: String,
    pub value: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct Ref {
    pub constructor: Box<Term>,
    pub meta_name: String,
    pub index: u64,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: Oper,
    pub rhs: Box<Term>,
}

#[derive(Debug, Clone)]
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

