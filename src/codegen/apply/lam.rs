use crate::codegen::apply::Codegen;
use crate::ir::apply::{Instruction, NameId, Position, Term};
use crate::ir::syntax::Lam as IRLam;

impl Codegen {
    pub fn build_lam(&mut self, expr: IRLam) -> Term {
        let IRLam {
            box value,
            parameter,
            erased,
            global_id,
        } = expr;

        let name = self.alloc_lam(global_id);
        let atom = Term::create_atom(Position::initial(&name));

        self.variables.push((parameter, atom)); // Push to the variable stack
        let value = self.build_term(value);
        self.variables.pop();

        if erased {
            self.instr(Instruction::link(Position::initial(&name), Term::erased()));
        }
        self.instr(Instruction::link(Position::new(&name, 1), value));

        Term::create_lam(Position::initial(&name))
    }

    /// Allocates a new lambda, or returns the existing one.
    pub fn alloc_lam(&mut self, global_id: NameId) -> String {
        if let Some(global_term) = self.lambdas.get(&global_id) {
            return global_term.clone();
        }

        let name = self.fresh_name("lam");
        self.instr(Instruction::binding(&name, Term::alloc(2)));

        if global_id != 0 {
            // FIXME: sanitizer still can't detect if a scope-less lambda doesn't use its bound
            //        variable, so we must write an Era() here. When it does, we can remove
            //        this line.
            self.instr(Instruction::link(Position::initial(&name), Term::erased()));
            self.lambdas.insert(global_id, name.clone());
        }

        name
    }
}
