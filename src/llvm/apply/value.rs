use inkwell::values::BasicValueEnum;

use crate::ir::apply::Value;

use super::Codegen;

impl<'a> Codegen<'a> {
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
}