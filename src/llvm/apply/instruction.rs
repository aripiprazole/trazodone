use crate::ir::apply::Instruction;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::IncrementCost => {
                self.call_void_std("hvm__increment_cost", &[]);
            }

            Instruction::Collect(_) => {}
            Instruction::Free(_free_instruction) => {}
            Instruction::Term(_term_instruction) => {}

            Instruction::Link(link_instruction) => {
                let position = self.build_position(link_instruction.position);
                let term = self.build_term(link_instruction.term);

                self.hvm__link(position, term);
            }

            Instruction::Let(let_instruction) => {
                let name = format!("let.{}", let_instruction.name);
                let ptr = self.builder.build_alloca(self.context.i64_type(), &name);

                self.names.insert(let_instruction.name, ptr.into());

                self.builder
                    .build_store(ptr, self.build_term(let_instruction.value));
            }

            Instruction::Println(_message) => {
                todo!("Instruction::Println")
            }

            // These should be handled on the [crate::codegen::apply::graph]
            Instruction::Return(..) | Instruction::Metadata(..) | Instruction::If(..) => {
                super::erased_step!(Instruction::Return)
            }
        }
    }
}
