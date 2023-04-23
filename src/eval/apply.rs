use crate::codegen::apply::binary::build_binary_op;
use crate::eval::{Context, Control, Eval, Object};
use crate::ir::apply::{
    Agent, Alloc, Block, Free, FunctionId, GetExt, GetNumber, GetPosition, GetTag, If, Instruction,
    Let, Link, LoadArgument, Position, PositionBinary, Tag, Term, Value, U60,
};
use crate::runtime::{
    hvm__alloc, hvm__create_app, hvm__create_binary, hvm__create_constructor, hvm__create_erased,
    hvm__create_function, hvm__create_lam, hvm__create_u60, hvm__create_var, hvm__free,
    hvm__get_ext, hvm__get_host, hvm__get_loc, hvm__get_number, hvm__get_tag, hvm__get_term,
    hvm__increment_cost, hvm__link, hvm__load_argument,
};

impl Eval for Position {
    type Output = u64;

    fn eval(self, context: &mut Context) -> Self::Output {
        unsafe {
            match self {
                Position::Named {
                    gate_index,
                    reference_name,
                } => {
                    let n = context
                        .variables
                        .get(&reference_name)
                        .unwrap_or_else(|| panic!("unknown {reference_name}"))
                        .as_u64();

                    n + gate_index.eval(context)
                }
                Position::Host => *hvm__get_host(context.reduce),
            }
        }
    }
}

impl Eval for PositionBinary {
    type Output = u64;

    fn eval(self, context: &mut Context) -> Self::Output {
        match self {
            PositionBinary::Con(value) => value,
            PositionBinary::Sum(a, b) => a.eval(context) + b.eval(context),
            PositionBinary::Sub(a, b) => a.eval(context) + b.eval(context),
            PositionBinary::Mul(a, b) => a.eval(context) + b.eval(context),
            PositionBinary::Div(a, b) => a.eval(context) + b.eval(context),
        }
    }
}

impl Eval for Term {
    type Output = Object;

    fn eval(self, context: &mut Context) -> Self::Output {
        unsafe {
            match self {
                Term::Tag(Tag::F60) => Object::U64(hvm::F60),
                Term::Tag(Tag::U60) => Object::U64(hvm::U60),
                Term::Tag(Tag::FUNCTION) => Object::U64(hvm::FUN),
                Term::Tag(Tag::CONSTRUCTOR) => Object::U64(hvm::CTR),
                Term::Tag(..) => todo!(),
                Term::ArityOf(_) => todo!(),
                Term::Ext(id, ..) => Object::U64(id),
                Term::TakeArgument(_) => todo!(),
                Term::NotFound(atom) => {
                    panic!("Atom not found: ({:?})", atom)
                }
                Term::Current => Object::U64(hvm__get_term(context.reduce)),
                Term::True => Object::Bool(true),
                Term::False => Object::Bool(false),
                Term::Alloc(Alloc { size }) => Object::U64(hvm__alloc(context.reduce, size)),
                Term::Agent(Agent { arguments, .. }) => {
                    let name = format!("agent_{}", context.variables.len() + 1);
                    let value = hvm__alloc(context.reduce, 1);
                    context
                        .variables
                        .insert(name.clone(), Object::U64(value.clone()));

                    for (i, argument) in arguments.iter().enumerate() {
                        let position = Position::new(&name, i as u64).eval(context);
                        let ptr = argument.clone().eval(context);
                        hvm__link(context.reduce, position, ptr.as_u64());
                    }

                    Object::U64(value)
                }
                Term::Equal(box lhs, box rhs) => {
                    let lhs = lhs.eval(context);
                    let rhs = rhs.eval(context);
                    Object::Bool(lhs == rhs)
                }
                Term::GetExt(GetExt { term }) => {
                    Object::U64(hvm__get_ext(term.eval(context).as_u64()))
                }
                Term::LoadArgument(LoadArgument {
                    box term,
                    argument_index,
                }) => {
                    let term = term.eval(context).as_u64();

                    Object::U64(hvm__load_argument(context.reduce, term, argument_index))
                }
                Term::GetNumber(GetNumber { box term }) => {
                    Object::U64(hvm__get_number(term.eval(context).as_u64()))
                }
                Term::GetTag(GetTag { box term }) => {
                    Object::U64(hvm__get_tag(term.eval(context).as_u64()))
                }
                Term::GetPosition(GetPosition { box term, position }) => {
                    Object::U64(hvm__get_loc(term.eval(context).as_u64(), position))
                }
                Term::Create(Value::Erased) => Object::U64(hvm__create_erased()),
                Term::Create(Value::U60(U60(value))) => Object::U64(hvm__create_u60(value)),
                Term::Create(Value::Function(FunctionId(_, id), position)) => {
                    Object::U64(hvm__create_function(id, position.eval(context)))
                }
                Term::Create(Value::Constructor(FunctionId(_, id), position)) => {
                    Object::U64(hvm__create_constructor(id, position.eval(context)))
                }
                Term::Create(Value::Lam(position)) => {
                    Object::U64(hvm__create_lam(position.eval(context)))
                }
                Term::Create(Value::Atom(position)) => {
                    Object::U64(hvm__create_var(position.eval(context)))
                }
                Term::Create(Value::App(position)) => {
                    Object::U64(hvm__create_app(position.eval(context)))
                }
                Term::Create(Value::Binary(binary, position)) => {
                    let operand = build_binary_op(binary.op);

                    Object::U64(hvm__create_binary(operand, position.eval(context)))
                }
                Term::Ref(name) => context
                    .variables
                    .get(&name)
                    .unwrap_or_else(|| panic!("Could not find variable {name}"))
                    .clone(),
                Term::LogicalOr(box lhs, box rhs) => {
                    let lhs = lhs.eval(context);
                    let rhs = rhs.eval(context);

                    if lhs.as_bool() {
                        lhs
                    } else {
                        rhs
                    }
                }
                Term::LogicalAnd(box lhs, box rhs) => {
                    let lhs = lhs.eval(context);
                    let rhs = rhs.eval(context);

                    if lhs.as_bool() && rhs.as_bool() {
                        lhs
                    } else {
                        Object::Bool(false)
                    }
                }
                Term::Create(value) => {
                    todo!("Cannot create value: {:?}", value)
                }
            }
        }
    }
}

impl Eval for Instruction {
    type Output = Control;

    fn eval(self, context: &mut Context) -> Self::Output {
        unsafe {
            match self {
                Instruction::Collect(_) => {}
                Instruction::If(If {
                    condition,
                    then,
                    otherwise,
                }) => {
                    let condition = condition.eval(context).as_bool();
                    if condition {
                        let mut then_context = context.clone();
                        for instruction in then.block {
                            if let Control::Break(value) = instruction.eval(&mut then_context) {
                                return Control::Break(value);
                            }
                        }
                    } else if let Some(otherwise) = otherwise {
                        let mut otherwise_context = context.clone();
                        for instruction in otherwise.block {
                            if let Control::Break(value) = instruction.eval(&mut otherwise_context)
                            {
                                return Control::Break(value);
                            }
                        }
                    }
                }
                Instruction::Let(Let { name, value }) => {
                    let value = value.eval(context);
                    context.variables.insert(name, value);
                }
                Instruction::Term(term) => {
                    term.eval(context).as_u64();
                }
                Instruction::Return(value) => {
                    return Control::Break(value.eval(context));
                }
                Instruction::IncrementCost => {
                    hvm__increment_cost(context.reduce);
                }
                Instruction::Link(Link { term, position }) => {
                    let term = term.eval(context).as_u64();
                    let position = position.eval(context);

                    hvm__link(context.reduce, position, term);
                }
                Instruction::Free(Free { position, arity }) => {
                    let position = position.eval(context).as_u64();

                    hvm__free(context.reduce, position, arity)
                }
                Instruction::Println(message) => {
                    println!("{}", message);
                }
                Instruction::Metadata(metadata) => {
                    for instruction in metadata.instructions {
                        if let Control::Break(value) = instruction.eval(context) {
                            return Control::Break(value);
                        }
                    }
                }
            }
        }

        Control::Continue
    }
}

impl Eval for Block {
    type Output = Object;

    fn eval(self, context: &mut Context) -> Self::Output {
        for instruction in self.block {
            if let Control::Break(value) = instruction.eval(context) {
                return value;
            }
        }

        Object::Bool(false)
    }
}
