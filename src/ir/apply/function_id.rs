pub type DebugName = Option<String>;

#[derive(Debug, Clone)]
pub struct FunctionId(pub DebugName, pub u64);

impl FunctionId {
    pub fn new(name: &str, id: u64) -> Self {
        FunctionId(Some(name.into()), id)
    }
}
