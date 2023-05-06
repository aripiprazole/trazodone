/// Represents the position of a term in the HVM heap.
#[derive(Debug, Clone)]
pub enum Position {
    /// A named position, the position is relative to a gate,
    /// where the gate is identified by its name and index.
    Named {
        reference_name: String,
        gate_index: u64,
    },
    /// A host position, the position is relative to the context's host.
    Host,
}

impl Position {
    /// Creates a new named position with the given name and index.
    pub fn new(reference_name: &str, gate_index: u64) -> Self {
        Self::Named {
            reference_name: reference_name.into(),
            gate_index,
        }
    }
    
    /// Creates a new named position with the initial gate index.
    pub fn initial(reference_name: &str) -> Self {
        Self::new(reference_name, 0)
    }
}
