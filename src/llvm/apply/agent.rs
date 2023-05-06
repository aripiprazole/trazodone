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

        for (index, argument) in agent.arguments.iter().enumerate() {
            let argument = self.build_term(argument.clone());
            let index = self.u64(index as u64);
            // Adds, to the index, the value of the agent.
            let llvm_index =
                self.builder
                    .build_int_add(value.into_int_value(), index.into_int_value(), "");
            self.hvm__link(llvm_index.into(), argument);
        }

        value
    }
}
