use crate::ir;

/// Represents a compiled rule group, composed by a name, a visit function and an apply function.
/// The apply function contains the compiled function code
/// with the function branches.
#[derive(Debug, Clone)]
pub struct RuleGroup {
    pub name: String,
    pub hvm_visit: ir::graph::BasicBlock<ir::visit::Instruction>,
    pub hvm_apply: ir::apply::Block,
}
