use hvm::syntax::Oper;

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

pub const fn build_binary_op(oper: Oper) -> u64 {
    match oper {
        Oper::Add => 0x0,
        Oper::Sub => 0x1,
        Oper::Mul => 0x2,
        Oper::Div => 0x3,
        Oper::Mod => 0x4,
        Oper::And => 0x5,
        Oper::Or => 0x6,
        Oper::Xor => 0x7,
        Oper::Shl => 0x8,
        Oper::Shr => 0x9,
        Oper::Ltn => 0xa,
        Oper::Lte => 0xb,
        Oper::Eql => 0xc,
        Oper::Gte => 0xd,
        Oper::Gtn => 0xe,
        Oper::Neq => 0xf,
    }
}
