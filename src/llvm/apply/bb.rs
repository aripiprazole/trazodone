use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::ir::apply::{ApplyBasicBlock, Instruction, Term, Value};
use crate::ir::graph::{Label, Terminator};
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

    pub fn build_terminator(&mut self, bb: ApplyBasicBlock, function: FunctionValue<'a>) {
        match bb.terminator {
            Terminator::Unreachable => {}
            Terminator::Debug(_) => {}
            Terminator::Jump(_) => {}
            Terminator::Return(value) => {
                self.builder.build_return(Some(&self.build_term(value)));
            }
            Terminator::Cond(cond, Label(then), Label(otherwise)) => {
                let old_bb = self.bb.unwrap();

                let then = bb.declared_blocks.get(&then).unwrap();
                let then = self.build_basic_block(function, then.clone());
                let otherwise = bb.declared_blocks.get(&otherwise).unwrap();
                let otherwise = self.build_basic_block(function, otherwise.clone());

                self.builder.position_at_end(old_bb);
                self.builder.build_conditional_branch(
                    self.build_term(cond).into_int_value(),
                    then,
                    otherwise,
                );
            }
        }
    }

    pub fn build_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::IncrementCost => {
                self.call_void_std("hvm__increment_cost", &[]);
            }
            Instruction::Collect(_) => {}
            Instruction::Free(_free_instruction) => {}
            Instruction::Link(_link_instruction) => {}
            Instruction::Term(_term_instruction) => {}

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

    pub fn build_value(&self, value: Value) -> BasicValueEnum {
        match value {
            Value::Dp0(_, _) => todo!(),
            Value::Dp1(_, _) => todo!(),
            Value::Argument(_) => todo!(),
            Value::Atom(_) => todo!(),
            Value::Lam(_) => todo!(),
            Value::App(_) => todo!(),
            Value::Super(_, _) => todo!(),
            Value::Binary(_, _) => todo!(),
            Value::U60(_) => todo!(),
            Value::F60(_) => todo!(),
            Value::Function(_, _) => todo!(),
            Value::Constructor(_, _) => todo!(),
            Value::Erased => todo!(),
        }
    }

    pub fn build_term(&self, term: Term) -> BasicValueEnum {
        match term {
            Term::Current => self.call_std("hvm__get_term", &[]),

            Term::Tag(tag) => self.context.i64_type().const_int(tag.id(), false).into(),

            Term::ArityOf(_) => todo!(),
            Term::TakeArgument(_) => todo!(),
            Term::LoadArgument(load_argument) => {
                let term = self.build_term(*load_argument.term);
                let argument_index = self
                    .context
                    .i64_type()
                    .const_int(load_argument.argument_index, false);

                self.call_std("hvm__load_argument", &[term.into(), argument_index.into()])
            }

            Term::GetExt(get_ext) => {
                self.call_direct("hvm__get_ext", &[self.build_term(*get_ext.term).into()])
            }
            Term::GetNumber(get_number) => {
                let term = self.build_term(*get_number.term);

                self.call_direct("hvm__get_number", &[term.into()])
            }
            Term::GetTag(get_tag) => {
                let term = self.build_term(*get_tag.term);
                self.call_direct("hvm__get_tag", &[term.into()])
            }
            Term::GetPosition(get_position) => {
                let term = self.build_term(*get_position.term);
                let position = self
                    .context
                    .i64_type()
                    .const_int(get_position.position, false);

                self.call_direct("hvm__get_position", &[term.into(), position.into()])
            }

            Term::Create(value) => self.build_value(value),
            Term::Alloc(alloc) => self.call_std(
                "hvm__alloc",
                &[self.context.i64_type().const_int(alloc.size, false).into()],
            ),
            Term::Agent(agent) => {
                let value = self.call_std(
                    "hvm__alloc",
                    &[self.context.i64_type().const_int(agent.arity, false).into()],
                );
                let alloca = self.builder.build_alloca(value.get_type(), "");
                self.builder.build_store(alloca, value);
                self.builder.build_load(self.context.i64_type(), alloca, "")
            }

            Term::Ext(id, _) => self.context.i64_type().const_int(id, false).into(),

            Term::Equal(box lhs, box rhs) => {
                let lhs = self.build_term(lhs);
                let rhs = self.build_term(rhs);

                self.builder
                    .build_int_compare(
                        inkwell::IntPredicate::EQ,
                        lhs.into_int_value(),
                        rhs.into_int_value(),
                        "",
                    )
                    .into()
            }
            Term::LogicalOr(box lhs, box rhs) => {
                let lhs = self.build_term(lhs);
                let rhs = self.build_term(rhs);

                self.call_direct("hvm__llvm_or", &[lhs.into(), rhs.into()])
            }
            Term::LogicalAnd(box lhs, box rhs) => {
                let lhs = self.build_term(lhs);
                let rhs = self.build_term(rhs);

                self.call_direct("hvm__llvm_and", &[lhs.into(), rhs.into()])
            }

            Term::Ref(reference) => {
                let alloca = *self
                    .names
                    .get(&reference)
                    .unwrap_or_else(|| panic!("Reference {:?} not found", reference,));

                self.builder
                    .build_load(self.context.i64_type(), alloca.into_pointer_value(), "")
            }

            Term::True => self.context.bool_type().const_int(1, false).into(),
            Term::False => self.context.bool_type().const_int(0, false).into(),

            Term::NotFound(..) => panic!("Term::NotFound can't be handled here"),
        }
    }
}
