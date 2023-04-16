use std::collections::HashMap;

use crate::ir::{
    Alloc, Block, Free, FunctionId, GetNumber, GetPosition, GetTag, If, Instruction, Let, Link,
    LoadArgument, Position, PositionBinary, Term, Value, U60,
};
use crate::runtime::{
    hvm__alloc, hvm__create_constructor, hvm__free, hvm__get_host, hvm__get_loc, hvm__get_number,
    hvm__get_tag, hvm__get_term, hvm__increment_cost, hvm__link, hvm__load_argument,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    U64(u64),
    Bool(bool),
    Pointer(*mut libc::c_void),
}

#[derive(Debug, Clone)]
pub struct Context {
    pub reduce_context: crate::runtime::ReduceContext,
    pub variables: HashMap<String, Object>,
}

pub trait Eval {
    type Output;

    fn eval(self, context: &mut Context) -> Self::Output;
}

impl Eval for Position {
    type Output = u64;

    fn eval(self, context: &mut Context) -> Self::Output {
        unsafe {
            match self {
                Position::Named { gate_index, .. } => gate_index.eval(context),
                Position::Host => *hvm__get_host(context.reduce_context),
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
                Term::Tag(_) => todo!(),
                Term::ArityOf(_) => todo!(),
                Term::GetExt(_) => todo!(),
                Term::Ext(_) => todo!(),
                Term::TakeArgument(_) => todo!(),
                Term::True => Object::Bool(true),
                Term::False => Object::Bool(false),
                Term::Equal(box lhs, box rhs) => {
                    let lhs = lhs.eval(context);
                    let rhs = rhs.eval(context);
                    Object::Bool(lhs == rhs)
                }
                Term::NotFound(atom) => {
                    panic!("Atom not found: ({:?})", atom)
                }
                Term::Current => Object::U64(hvm__get_term(context.reduce_context)),
                Term::LoadArgument(LoadArgument { argument_index }) => {
                    Object::U64(hvm__load_argument(context.reduce_context, argument_index))
                }
                Term::Alloc(Alloc { size }) => {
                    Object::U64(hvm__alloc(context.reduce_context, size))
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
                Term::Create(Value::U60(U60(value))) => {
                    Object::U64(hvm__alloc(context.reduce_context, value))
                }
                Term::Create(Value::Constructor(FunctionId(_name, id), position)) => {
                    Object::U64(hvm__create_constructor(id, position.eval(context)))
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

pub enum Control {
    Break(Object),
    Continue,
}

impl Eval for Instruction {
    type Output = Control;

    fn eval(self, context: &mut Context) -> Self::Output {
        unsafe {
            match self {
                Instruction::Collect(_) => {}
                Instruction::WHNF(_) => {}
                Instruction::If(If {
                    condition,
                    then,
                    otherwise,
                }) => {
                    let condition = condition.eval(context).as_bool();
                    if condition {
                        let mut then_context = context.clone();
                        for instruction in then.instructions {
                            if let Control::Break(value) = instruction.eval(&mut then_context) {
                                return Control::Break(value);
                            }
                        }
                    } else if let Some(otherwise) = otherwise {
                        let mut otherwise_context = context.clone();
                        for instruction in otherwise.instructions {
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
                    hvm__increment_cost(context.reduce_context);
                }
                Instruction::Link(Link { term, position }) => {
                    let a = term.eval(context).as_u64();
                    let b = position.eval(context);

                    hvm__link(context.reduce_context, a, b);
                }
                Instruction::Free(Free { position, arity }) => {
                    let position = position.eval(context).as_u64();

                    hvm__free(context.reduce_context, position, arity)
                }
            }
        }

        Control::Continue
    }
}

impl Eval for Block {
    type Output = Object;

    fn eval(self, context: &mut Context) -> Self::Output {
        for instruction in self.instructions {
            if let Control::Break(value) = instruction.eval(context) {
                return value;
            }
        }

        panic!("No return statement found")
    }
}

/**
function apply(ctx) {
  let arg0 = load_argument(ctx, 0);
  let arg1 = load_argument(ctx, 1);
  if (get_tag(arg0) == ...) {
    return caralhao_pequeno();
  }
  if (get_tag(arg0) == ...) {
    return caralhao_grande();
  }
  return heavy_computation();
}

apply(ctx);
*/

impl Object {
    pub fn as_u64(&self) -> u64 {
        match self {
            Object::U64(value) => *value,
            _ => panic!("Expected u64, got {:?}", self),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Object::Bool(value) => *value,
            _ => panic!("Expected bool, got {:?}", self),
        }
    }
}
