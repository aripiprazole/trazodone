use inkwell::values::BasicValueEnum;

use crate::ir::apply::Position;
use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_position(&self, position: Position) -> BasicValueEnum {
        match position {
            Position::Named {
                gate_index,
                reference_name,
            } => {
                let alloca = *self
                    .names
                    .get(&reference_name)
                    .unwrap_or_else(|| panic!("Position reference {:?} not found", reference_name));

                let value = self.builder.build_load(
                    self.context.i64_type(),
                    alloca.into_pointer_value(),
                    "",
                );

                let index = self.u64(gate_index as u64);
                let index =
                    self.builder
                        .build_int_add(value.into_int_value(), index.into_int_value(), "");

                index.into()
            }
            Position::Host => self.hvm__get_host_value(),
        }
    }
}
