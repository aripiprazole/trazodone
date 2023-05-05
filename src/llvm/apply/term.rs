use inkwell::values::BasicValueEnum;

use crate::ir::apply::{GetExt, GetNumber, GetPosition, GetTag, LoadArgument, Term};

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_term(&self, term: Term) -> BasicValueEnum {
        match term {
            Term::Current => self.hvm__get_term(),

            Term::Tag(tag) => self.context.i64_type().const_int(tag.id(), false).into(),

            Term::ArityOf(_) => todo!(),
            Term::TakeArgument(_) => todo!(),

            Term::GetExt(get_ext) => self.build_get_ext(get_ext),
            Term::GetNumber(get_number) => self.build_get_number(get_number),
            Term::GetTag(get_tag) => self.build_get_tag(get_tag),
            Term::GetPosition(get_position) => self.build_get_position(get_position),
            Term::LoadArgument(load_argument) => self.build_load_argument(load_argument),

            Term::Create(value) => self.build_value(value),
            Term::Alloc(alloc) => self.hvm__alloc(self.u64(alloc.size)),
            Term::Agent(agent) => self.build_agent(agent),

            Term::Ext(id, _) => self.context.i64_type().const_int(id, false).into(),

            Term::Equal(box lhs, box rhs) => self.build_equal(lhs, rhs),
            Term::LogicalOr(box lhs, box rhs) => self.build_logical_or(lhs, rhs),
            Term::LogicalAnd(box lhs, box rhs) => self.build_logical_and(lhs, rhs),

            Term::Ref(reference) => self.build_ref(reference),

            Term::True => self.context.bool_type().const_int(1, false).into(),
            Term::False => self.context.bool_type().const_int(0, false).into(),

            Term::NotFound(..) => panic!("Term::NotFound can't be handled here"),
        }
    }

    pub fn build_get_ext(&self, term: GetExt) -> BasicValueEnum {
        let llvm = self.build_term(*term.term);
        self.hvm__get_ext(llvm)
    }

    pub fn build_get_number(&self, term: GetNumber) -> BasicValueEnum {
        let llvm = self.build_term(*term.term);
        self.hvm__get_number(llvm)
    }

    pub fn build_get_tag(&self, term: GetTag) -> BasicValueEnum {
        let llvm = self.build_term(*term.term);
        self.hvm__get_tag(llvm)
    }

    pub fn build_get_position(&self, term: GetPosition) -> BasicValueEnum {
        let llvm = self.build_term(*term.term);
        let position = self.u64(term.position);

        self.hvm__get_loc(llvm, position)
    }

    pub fn build_load_argument(&self, term: LoadArgument) -> BasicValueEnum {
        let llvm = self.build_term(*term.term);
        let argument_index = self.u64(term.argument_index);

        self.hvm__load_argument(llvm, argument_index)
    }

    pub fn build_equal(&self, lhs: Term, rhs: Term) -> BasicValueEnum {
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

    /// TODO: use native LLVM constructs, instead of calling a function.
    pub fn build_logical_or(&self, lhs: Term, rhs: Term) -> BasicValueEnum {
        let lhs = self.build_term(lhs);
        let rhs = self.build_term(rhs);

        self.hvm__llvm_or(lhs, rhs)
    }

    /// TODO: use native LLVM constructs, instead of calling a function.
    pub fn build_logical_and(&self, lhs: Term, rhs: Term) -> BasicValueEnum {
        let lhs = self.build_term(lhs);
        let rhs = self.build_term(rhs);

        self.hvm__llvm_and(lhs, rhs)
    }

    pub fn build_ref(&self, reference: String) -> BasicValueEnum {
        let alloca = *self
            .names
            .get(&reference)
            .unwrap_or_else(|| panic!("Reference {:?} not found", reference,));

        self.builder
            .build_load(self.context.i64_type(), alloca.into_pointer_value(), "")
    }
}
