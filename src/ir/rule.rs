use crate::ir;

#[derive(Debug, Clone)]
pub struct RuleGroup {
    pub name: String,
    pub hvm_visit: ir::graph::BasicBlock<ir::visit::Instruction>,
    pub hvm_apply: ir::apply::Block,
}
