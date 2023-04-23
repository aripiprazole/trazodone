use crate::ir::apply::NameId;

pub type DebugName = Option<String>;

/// A unique identifier for a function, the name
/// is only used for debugging purposes, and the
/// id is used to identify the function.
#[derive(Debug, Clone)]
pub struct FunctionId(pub DebugName, pub NameId);

impl FunctionId {
    /// Creates a new function identifier.
    pub fn new(name: &str, id: u64) -> Self {
        FunctionId(Some(name.into()), id)
    }
}
