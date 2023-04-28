use inkwell::basic_block::BasicBlock;
use inkwell::values::FunctionValue;

use crate::ir::apply::ApplyBasicBlock;
use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_basic_block(
        &mut self,
        function: FunctionValue<'a>,
        bb: ApplyBasicBlock,
    ) -> BasicBlock<'a> {
        let llvm_bb = self.context.append_basic_block(function, &bb.label);
        self.builder.position_at_end(llvm_bb);
        self.bb = Some(llvm_bb);

        for instruction in &bb.instructions {
            self.build_instruction(instruction.clone());
        }

        self.build_terminator(bb, function);

        llvm_bb
    }
}
