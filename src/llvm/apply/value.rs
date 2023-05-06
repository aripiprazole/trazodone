use inkwell::values::BasicValueEnum;

use crate::ir::apply::{build_binary_op, Value, F60, U60};

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_value(&self, value: Value) -> BasicValueEnum {
        match value {
            // TODO: super position stuff
            Value::Dp0(..) => todo!(),
            Value::Dp1(..) => todo!(),
            Value::Super(..) => todo!(),

            //
            Value::Argument(..) => todo!(),
            Value::Atom(position) => self.hvm__create_var(self.build_position(position)),
            Value::Lam(position) => self.hvm__create_lam(self.build_position(position)),
            Value::App(position) => self.hvm__create_app(self.build_position(position)),
            Value::U60(U60(value)) => self.hvm__create_u60(self.u64(value)),
            Value::F60(F60(value)) => self.hvm__create_f60(self.f64(value)),
            Value::Binary(binary, position) => {
                let operand = self.u64(build_binary_op(binary.op));
                let position = self.build_position(position);

                self.hvm__create_binary(operand, position)
            }
            Value::Function(fn_id, position) => {
                let fn_id = self.u64(fn_id.1);
                let position = self.build_position(position);
                self.hvm__create_function(fn_id, position)
            }
            Value::Constructor(fn_id, position) => {
                let fn_id = self.u64(fn_id.1);
                let position = self.build_position(position);
                self.hvm__create_constructor(fn_id, position)
            }
            Value::Erased => self.hvm__create_erased(),
        }
    }
}
