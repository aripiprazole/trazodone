use std::ops::Deref;

use hvm::syntax::Oper;

use crate::tree;

#[derive(Default, Clone)]
pub struct Block {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct RuleGroup {
    pub name: String,
    pub hvm_visit: Block,
    pub hvm_apply: Block,
}

#[derive(Debug, Clone)]
pub struct Color(pub u64);

#[derive(Debug, Clone)]
pub struct U60(pub u64);

#[derive(Debug, Clone)]
pub struct F60(pub f64);

#[derive(Debug, Clone)]
pub struct FunctionId(pub String);

#[derive(Debug, Clone)]
pub enum Position {
    Named {
        reference_name: String,
        gate_index: u64,
    },
    Host,
}

#[derive(Debug, Clone)]
pub struct IntValue(pub Value);

#[derive(Debug, Clone)]
pub struct LoadArgument {
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
pub struct WHNF {
    pub strictness_index: u64,
}

#[derive(Debug, Clone)]
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
    Ext(String),
    Equal(Box<Term>, Box<Term>),
    LogicalOr(Box<Term>, Box<Term>),
    LogicalAnd(Box<Term>, Box<Term>),
    Ref(String),
    True,
    False,
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

pub type Result<T> = std::result::Result<T, String>;

pub type Insertion<'a> = &'a mut Vec<Instruction>;
pub type FreeIndex = u64;
pub type FreeArity = u64;
pub type FreeVec = Vec<(FreeIndex, FreeArity)>;

impl RuleGroup {
    pub fn specialize(group: tree::RuleGroup) -> Result<Self> {
        Ok(Self {
            name: group.name.clone(),
            hvm_visit: Block::default(),
            hvm_apply: Self::specialize_hvm_apply(&group)?,
        })
    }

    pub fn specialize_hvm_apply(group: &tree::RuleGroup) -> Result<Block> {
        let rules = group.rules.clone();
        let strict_parameters = group.strict_parameters.clone();

        let mut hvm_apply = vec![];

        if rules.is_empty() {
            return Err("no rules".into());
        }

        for i in 0..strict_parameters.len() {
            hvm_apply.push(Instruction::binding(
                &format!("arg_{i}"),
                Term::load_arg(i as u64),
            ));
        }

        // TODO: superpose

        for rule in rules {
            let free = rule
                .parameters
                .iter()
                .enumerate()
                .filter(|(_, parameter)| matches!(parameter, tree::Parameter::Constructor(..)))
                .map(|(i, parameter)| match parameter {
                    tree::Parameter::Constructor(constructor) => (i as u64, constructor.arity),
                    _ => unreachable!(),
                })
                .collect();

            let mut match_rule = Term::True;

            for (i, parameter) in rule.parameters.iter().cloned().enumerate() {
                match_rule = Term::logical_and(
                    match_rule,
                    Self::build_pattern_matching(group, i, parameter),
                );
            }

            // TODO: Collect unused variables
            if match_rule.is_true() {
                let done = Self::build_rule(&mut hvm_apply, &rule);
                Self::build_link(&mut hvm_apply, done);
                Self::build_free(&mut hvm_apply, &group, free);
            } else {
                let mut then = Block::with(Instruction::IncrementCost);
                let done = Self::build_rule(&mut hvm_apply, &rule);
                Self::build_link(&mut then.instructions, done);
                Self::build_free(&mut then.instructions, &group, free);

                hvm_apply.push(Instruction::If(If {
                    condition: match_rule,
                    then,
                    otherwise: None,
                }));
            }
        }

        Ok(Block::new(hvm_apply))
    }

    pub fn build_rule(_block: Insertion, _rule: &tree::Rule) -> Term {
        // TODO
        Term::True
    }

    pub fn build_link(block: Insertion, done: Term) {
        block.push(Instruction::link(Position::Host, done));
    }

    pub fn build_free(block: Insertion, rule: &tree::RuleGroup, free: FreeVec) {
        let mut free = free
            .iter()
            .map(|(index, arity)| {
                let argument = Term::reference(&format!("arg_{index}"));

                (Term::get_position(argument, 0), *arity)
            })
            .collect::<Vec<_>>();

        free.push((
            Term::get_position(Term::Current, 0),
            rule.strict_parameters.len() as u64,
        ));

        for must_free in free {
            block.push(Instruction::Free(Free {
                position: must_free.0,
                arity: must_free.1,
            }));
        }
    }

    pub fn build_pattern_matching(
        group: &tree::RuleGroup,
        i: usize,
        parameter: tree::Parameter,
    ) -> Term {
        let argument = Term::reference(&format!("arg_{i}"));

        match parameter {
            tree::Parameter::U60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::U60)),
                Term::equal(Term::get_num(argument), Term::create_u60(value)),
            ),
            tree::Parameter::F60(value) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::F60)),
                Term::equal(Term::get_num(argument), Term::create_f60(value)),
            ),
            tree::Parameter::Constructor(tree::Constructor { name, .. }) => Term::logical_and(
                Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::CONSTRUCTOR)),
                Term::equal(Term::get_num(argument), Term::ext(&name)),
            ),
            tree::Parameter::Atom(..) if group.strict_parameters[i] => {
                // TODO: hoas for kind2

                Term::logical_or(
                    Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::CONSTRUCTOR)),
                    Term::logical_or(
                        Term::equal(Term::get_tag(argument.clone()), Term::Tag(Tag::U60)),
                        Term::equal(Term::get_tag(argument), Term::Tag(Tag::F60)),
                    ),
                )
            }
            _ => Term::True,
        }
    }
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

    pub fn get_position(term: Term, position: u64) -> Self {
        Term::GetPosition(GetPosition {
            term: term.into(),
            position,
        })
    }

    pub fn load_arg(argument_index: u64) -> Self {
        Term::LoadArgument(LoadArgument { argument_index })
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

    pub fn ext(ext: &str) -> Self {
        Term::Ext(ext.into())
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

impl Deref for Block {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        match self {
            Block { instructions } => instructions,
        }
    }
}

impl Block {
    pub fn with(instruction: Instruction) -> Self {
        Self {
            instructions: vec![instruction],
        }
    }

    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}
