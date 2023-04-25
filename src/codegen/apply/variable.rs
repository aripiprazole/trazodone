use crate::codegen::apply::Codegen;
use crate::ir::apply::Term;
use crate::ir::apply::Variable;

impl Variable {
    pub fn as_name(&self) -> String {
        match self.field_index {
            Some(field_index) => format!("arg{index}_{field_index}", index = self.index),
            None => format!("arg{index}", index = self.index),
        }
    }

    pub fn as_simple_name(&self) -> String {
        match self.field_index {
            Some(field_index) => format!("x{index}_{field_index}", index = self.index),
            None => format!("x{index}", index = self.index),
        }
    }

    pub fn as_term(&self) -> Term {
        Term::reference(&self.as_name())
    }
}

impl Codegen {
    pub fn variable_as_tuple(&mut self, variable: &Variable) -> (String, Term) {
        let name = "*";

        let term = match variable.field_index {
            Some(field_index) => self
                .get_argument(variable.index as usize)
                .get_field(field_index as usize),
            None => self.get_argument(variable.index as usize).unbox(),
        };

        (name.into(), term)
    }
}
