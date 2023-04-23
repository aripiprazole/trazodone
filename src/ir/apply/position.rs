#[derive(Debug, Clone)]
pub enum PositionBinary {
    Con(u64),
    Sum(Box<PositionBinary>, Box<PositionBinary>),
    Sub(Box<PositionBinary>, Box<PositionBinary>),
    Mul(Box<PositionBinary>, Box<PositionBinary>),
    Div(Box<PositionBinary>, Box<PositionBinary>),
}

#[derive(Debug, Clone)]
pub enum Position {
    Named {
        reference_name: String,
        gate_index: PositionBinary,
    },
    Host,
}

impl Position {
    pub fn new(reference_name: &str, gate_index: u64) -> Self {
        Self::Named {
            reference_name: reference_name.into(),
            gate_index: PositionBinary::Con(gate_index),
        }
    }

    pub fn binary(reference_name: &str, gate_index: PositionBinary) -> Self {
        Self::Named {
            reference_name: reference_name.into(),
            gate_index,
        }
    }

    pub fn initial(reference_name: &str) -> Self {
        Self::new(reference_name, 0)
    }
}
