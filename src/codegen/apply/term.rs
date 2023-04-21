use crate::codegen::apply::Codegen;
use crate::ir::apply::*;
use crate::ir::syntax;

impl Codegen {
    pub fn build_term(&mut self, term: syntax::Term) -> Term {
        use crate::ir::syntax::Term::*;

        match term {
            U60(u60) => Term::create_u60(u60),
            F60(f60) => Term::create_f60(f60),
            Let(let_expr) => self.build_let(let_expr.name, *let_expr.value, *let_expr.body),
            Lam(lam_expr) => self.build_lam(
                lam_expr.parameter,
                lam_expr.global_id,
                lam_expr.erased,
                *lam_expr.value,
            ),
            Binary(binary_expr) => {
                self.build_binary(*binary_expr.lhs, binary_expr.op, *binary_expr.rhs)
            }
            Atom(atom_expr) => {
                self.build_atom(atom_expr.name, atom_expr.index, atom_expr.field_index)
            }
            App(app) => match app.global_name {
                Some(global_name) if app.is_function => self.build_call(app.arguments, global_name),
                Some(global_name) => self.build_constructor(app.arguments, global_name),
                None => self.build_app(*app.callee, app.arguments),
            },
            Duplicate(_) => todo!(),
            Super(_) => todo!(),
        }
    }

    fn build_lam(
        &mut self,
        parameter: String,
        global_id: u64,
        erased: bool,
        value: syntax::Term,
    ) -> Term {
        let name = self.alloc_lam(global_id);
        self.variables
            .push((parameter, Term::create_atom(Position::initial(&name))));
        let value = self.build_term(value);
        self.variables.pop();
        if erased {
            self.instr(Instruction::link(
                Position::initial(&name),
                Term::create_erased(),
            ));
        }
        self.instr(Instruction::link(Position::new(&name, 1), value));

        Term::create_lam(Position::initial(&name))
    }

    fn build_atom(&mut self, name: String, index: u64, field_index: Option<u64>) -> Term {
        match self.variables.get(index as usize) {
            Some((_, value)) => value.clone(),
            // TODO: fix this simple workaround
            None => self
                .variables
                .iter()
                .find(|(var_name, _)| var_name == &name)
                .map(|(_, value)| value.clone())
                .unwrap_or_else(|| {
                    Term::NotFound(syntax::Atom {
                        name,
                        index,
                        field_index,
                    })
                }),
        }
    }

    fn build_let(&mut self, name: String, value: syntax::Term, body: syntax::Term) -> Term {
        let value = self.build_term(value);
        self.variables.push((name, value));
        let body = self.build_term(body);
        self.variables.pop();

        body
    }
}
