use inkwell::values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue};

use crate::ir::apply::{ApplyBasicBlock, Instruction, Term};
use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_basic_block(&self, function: FunctionValue, bb: ApplyBasicBlock) {
        let ctx = function.get_first_param().unwrap();
        let llvm_bb = self.context.append_basic_block(function, &bb.label);
        self.builder.position_at_end(llvm_bb);

        for instruction in bb.instructions {
            match instruction {
                Instruction::IncrementCost => {
                    self.builder.build_direct_call(
                        self.module.get_function("hvm__increment_cost").unwrap(),
                        &[ctx.into()],
                        "",
                    );
                }
                Instruction::Collect(_) => {}
                Instruction::Free(_free_instruction) => {}
                Instruction::Link(_link_instruction) => {}
                Instruction::Term(_term_instruction) => {}

                Instruction::Let(let_instruction) => {
                    let name = format!("let.{}", let_instruction.name);
                    let ptr = self
                        .builder
                        .build_alloca(self.context.i64_type(), &name);

                    self.builder
                        .build_store(ptr, self.build_term(ctx, let_instruction.value));
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

    pub fn build_term<'b>(&'b self, ctx: BasicValueEnum<'b>, term: Term) -> BasicValueEnum {
        match term {
            Term::Current => self
                .builder
                .build_direct_call(
                    self.module.get_function("hvm__get_term").unwrap(),
                    &[ctx.into()],
                    "",
                )
                .try_as_basic_value()
                .unwrap_left(),

            Term::Tag(_) => todo!(),
            Term::ArityOf(_) => todo!(),
            Term::TakeArgument(_) => todo!(),
            Term::LoadArgument(load_argument) => {
                let term = self.build_term(ctx, *load_argument.term);

                self.builder
                    .build_direct_call(
                        self.module.get_function("hvm__load_argument").unwrap(),
                        &[
                            ctx.into(),
                            term.into(),
                            self.context.i64_type().const_int(0, false).into(),
                        ],
                        "",
                    )
                    .try_as_basic_value()
                    .unwrap_left()
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
