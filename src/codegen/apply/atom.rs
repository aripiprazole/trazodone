use crate::codegen::apply::Codegen;
use crate::ir::apply::Term;
use crate::ir::syntax::Atom as IRAtom;

impl Codegen {
    pub fn build_atom(&mut self, expr: IRAtom) -> Term {
        let IRAtom {
            name,
            index,
            field_index,
        } = expr;

        let (_, term) = self.variables.get(index as usize).unwrap_or_else(|| {
            panic!(
                "Variable not found: {:?} (index: {}, field_index: {:?})",
                &name, index, field_index
            )
        });

        term.clone()
    }
}
