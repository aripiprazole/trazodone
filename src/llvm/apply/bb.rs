use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::ir::apply::{ApplyBasicBlock, Instruction, Term};
use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_basic_block(&mut self, function: FunctionValue<'a>, bb: ApplyBasicBlock) {
        let llvm_bb = self.context.append_basic_block(function, &bb.label);
        self.builder.position_at_end(llvm_bb);
        self.bb = Some(llvm_bb);

        for instruction in bb.instructions {
            match instruction {
                Instruction::IncrementCost => {
                    self.call_std("hvm__increment_cost", &[]);
                }
                Instruction::Collect(_) => {}
                Instruction::Free(_free_instruction) => {}
                Instruction::Link(_link_instruction) => {}
                Instruction::Term(_term_instruction) => {}

                Instruction::Let(let_instruction) => {
                    let name = format!("let.{}", let_instruction.name);
                    let ptr = self.builder.build_alloca(self.context.i64_type(), &name);

                    self.builder
                        .build_store(ptr, self.build_term(let_instruction.value));
                }

                Instruction::Println(_message) => {
                    todo!("Instruction::Println")
                }

                // These should be handled on the [crate::codegen::apply::graph]
                Instruction::Return(_) => {
                    panic!("Instruction::Metadata should be handled by the if builder")
                }

                Instruction::Metadata(..) => {
                    panic!("Instruction::Metadata should be handled by the if builder")
                }
                Instruction::If(..) => {
                    panic!("Instruction::If should be handled by the if builder")
                }
            }
        }
        self.builder
            .build_return(Some(&self.context.bool_type().const_int(1, false)));
    }

    pub fn build_term(&self, term: Term) -> BasicValueEnum {
        match term {
            Term::Current => self.call_std("hvm__get_term", &[]),

            Term::Tag(_) => todo!(),
            Term::ArityOf(_) => todo!(),
            Term::TakeArgument(_) => todo!(),
            Term::LoadArgument(load_argument) => {
                let term = self.build_term(*load_argument.term);

                self.call_std(
                    "hvm__load_argument",
                    &[
                        term.into(),
                        self.context
                            .i64_type()
                            .const_int(load_argument.argument_index, false)
                            .into(),
                    ],
                )
            }
            Term::GetExt(_) => todo!(),
            Term::GetNumber(_) => todo!(),
            Term::GetTag(_) => todo!(),
            Term::GetPosition(_) => todo!(),
            Term::Create(_) => todo!(),
            Term::Alloc(_) => todo!(),
            Term::Agent(_) => todo!(),
            Term::Ext(_, _) => todo!(),
            Term::Equal(_, _) => todo!(),
            Term::LogicalOr(_, _) => todo!(),
            Term::LogicalAnd(_, _) => todo!(),
            Term::Ref(_) => todo!(),

            Term::True => self.context.bool_type().const_int(1, false).into(),
            Term::False => self.context.bool_type().const_int(0, false).into(),

            Term::NotFound(..) => panic!("Term::NotFound can't be handled here"),
        }
    }
}
