use inkwell::values::BasicValueEnum;

use crate::ir::apply::Agent;
use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    /// Build an agent, which is a combination of [Term::alloc] and [Instruction::link].
    ///
    /// This is a special case because it requires an alloca, and it's better to
    /// do that in a special function, because, this will desugar the agent into
    /// a [Term::alloc] and [Instruction::link] anyway.
    pub fn build_agent(&self, agent: Agent) -> BasicValueEnum {
        let value = self.hvm__alloc(self.u64(agent.arity));
        let alloca = self.builder.build_alloca(value.get_type(), "");
        self.builder.build_store(alloca, value);
        self.builder.build_load(self.context.i64_type(), alloca, "")
    }
}
