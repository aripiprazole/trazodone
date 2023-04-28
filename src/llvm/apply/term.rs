use inkwell::values::BasicValueEnum;

use crate::ir::apply::Term;

use super::Codegen;

impl<'a> Codegen<'a> {
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
