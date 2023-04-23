use crate::codegen::apply::Codegen;
use crate::ir::apply::*;
use crate::ir::syntax;
use crate::ir::syntax::{Atom as IRAtom, Lam as IRLam, Let as IRLet, Term as IRTerm};

impl Codegen {
    pub fn build_term(&mut self, term: IRTerm) -> Term {
        use IRTerm::*;

        match term.clone() {
            U60(u60) => Term::create_u60(u60),
            F60(f60) => Term::create_f60(f60),
            Atom(atom_expr) => self.build_atom(atom_expr),
            Let(let_expr) => self.build_let(let_expr),
            Lam(lam_expr) => self.with_metadata(term, |bb, _| {
                bb.build_lam(lam_expr)
            }),
            Binary(binary_expr) => self.with_metadata(term, |bb, _| {
                bb.build_binary(binary_expr)
            }),
            App(app) => self.with_metadata(term, |bb, _| {
                match app.global_name {
                    Some(global_name) if app.is_function => bb.build_call(app.arguments, global_name),
                    Some(global_name) => bb.build_constructor(app.arguments, global_name),
                    None => bb.build_app(*app.callee, app.arguments),
                }
            }),
            Duplicate(_) => todo!(),
            Super(_) => todo!(),
        }
    }

    fn build_lam(&mut self, expr: IRLam) -> Term {
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

    fn build_let(&mut self, expr: IRLet) -> Term {
        let IRLet {
            name,
            box value,
            box body,
        } = expr;

        let binding = self.build_term(value);
        self.variables.push((name, binding));
        let body = self.build_term(body);
        self.variables.pop();

        body
    }

    fn build_atom(&mut self, expr: IRAtom) -> Term {
        let IRAtom {
            name,
            index,
            field_index,
        } = expr;
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
}
