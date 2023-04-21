use std::collections::HashMap;

pub mod apply;
pub mod graph;
pub mod visit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    U64(u64),
    Bool(bool),
    Pointer(*mut libc::c_void),
}

#[derive(Debug, Clone)]
pub struct Context {
    pub reduce: crate::runtime::ReduceContext,
    pub variables: HashMap<String, Object>,
}

#[derive(Debug, Clone)]
pub enum Control {
    Break(Object),
    Continue,
}

pub trait Eval {
    type Output;

    fn eval(self, context: &mut Context) -> Self::Output;
}

impl Context {
    pub fn new(reduce: crate::runtime::ReduceContext) -> Self {
        Self {
            reduce,
            variables: HashMap::new(),
        }
    }
}

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

    pub fn as_ptr<T: Sized>(&self) -> *mut T {
        match self {
            Object::Pointer(value) => unsafe {
                std::mem::transmute::<*mut libc::c_void, *mut T>(*value)
            },
            _ => panic!("Expected ptr, got {:?}", self),
        }
    }
}
