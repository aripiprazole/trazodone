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
