use hvm::syntax::Oper;

/// Represents the builtin HVM binary operations
/// that can be applied to terms.
#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
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
