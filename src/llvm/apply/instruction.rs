use crate::ir::apply::{Free, Instruction, Let, Link};

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::IncrementCost => {
                self.hvm__increment_cost();
            }
            Instruction::Term(term) => {
                self.build_term(term);
            }

            Instruction::Collect(_collect_instruction) => {}
            Instruction::Free(free_instruction) => self.build_free(free_instruction),
            Instruction::Link(link_instruction) => self.build_link(link_instruction),
            Instruction::Let(let_instruction) => self.build_let(let_instruction),

            Instruction::Println(_message) => {
                todo!("Instruction::Println")
            }

            // These should be handled on the [crate::codegen::apply::graph]
            Instruction::Return(..) | Instruction::Metadata(..) | Instruction::If(..) => {
                super::erased_step!(Instruction::Return)
            }
        }
    }

    pub fn build_free(&mut self, instruction: Free) {
        let position = self.build_term(instruction.position);
        let arity = self.u64(instruction.arity);

        self.hvm__free(position, arity);
    }

    pub fn build_link(&mut self, instruction: Link) {
        let position = self.build_position(instruction.position);
        let term = self.build_term(instruction.term);

        self.hvm__link(position, term);
    }

    pub fn build_let(&mut self, instruction: Let) {
        let name = format!("let.{}", instruction.name);
        let ptr = self.builder.build_alloca(self.context.i64_type(), &name);

        self.names.insert(instruction.name, ptr.into());

        self.builder
            .build_store(ptr, self.build_term(instruction.value));
    }
}
