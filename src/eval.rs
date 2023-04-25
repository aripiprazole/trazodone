use fxhash::FxHashMap;

pub mod apply;
pub mod graph;
pub mod visit;

/// Evaluation object, that can be used to control the evaluation of the HVM terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    U64(u64),
    Bool(bool),
    Pointer(*mut libc::c_void),
}

#[derive(Debug, Clone)]
pub struct Context {
    /// HVM internal reduce context.
    pub reduce: crate::runtime::ReduceContext,

    /// Local variables evaluation context.
    pub variables: FxHashMap<String, Object>,
}

/// The `Control` enum is used to control the evaluation of the HVM terms.
#[derive(Debug, Clone)]
pub enum Control {
    /// Breaks the evaluation, and returns value to the [Eval] trait.
    Break(Object),

    /// Continues the evaluation.
    Continue,
}

/// The `Eval` trait is used to evaluate the HVM terms.
pub trait Eval {
    type Output;

    fn eval(self, context: &mut Context) -> Self::Output;
}

impl Context {
    pub fn new(reduce: crate::runtime::ReduceContext) -> Self {
        Self {
            reduce,
            variables: FxHashMap::default(),
        }
    }
}

impl Object {
    /// Returns the object as a u64.
    pub fn as_u64(&self) -> u64 {
        match self {
            Object::U64(value) => *value,
            _ => panic!("Expected u64, got {:?}", self),
        }
    }

    /// Returns the object as a bool.
    pub fn as_bool(&self) -> bool {
        match self {
            Object::Bool(value) => *value,
            _ => panic!("Expected bool, got {:?}", self),
        }
    }

    /// Returns the object as a pointer. Cast to [*mut T]
    pub fn as_ptr<T: Sized>(&self) -> *mut T {
        match self {
            Object::Pointer(value) => unsafe {
                std::mem::transmute::<*mut libc::c_void, *mut T>(*value)
            },
            _ => panic!("Expected ptr, got {:?}", self),
        }
    }
}
