use crate::codegen::apply::Codegen;
use crate::ir::apply::{Instruction, Position, Term};
use crate::ir::syntax::Binary as IRBinary;

impl Codegen {
    pub fn build_binary(&mut self, expr: IRBinary) -> Term {
        let IRBinary {
            op,
            box lhs,
            box rhs,
        } = expr;

        let binary = self.fresh_name("binary");

        let lhs = self.build_term(lhs);
        let rhs = self.build_term(rhs);

        // TODO: Optimization: do inline operation, avoiding Op2 allocation, when operands are already number
        let binary_alloc = self.make_agent(|builder| {
            builder.push(lhs.clone());
            builder.push(rhs.clone());
        });

        self.instr(Instruction::binding(&binary, binary_alloc));

        Term::create_binary(lhs, op, rhs, Position::initial(&binary))
    }
}
