use itertools::Itertools;
use rand::Rng;

use crate::ir::apply::{Block, Instruction, Term};
use crate::ir::graph::{BasicBlock, HasTerm, Label, Terminator};

impl Block {
    /// Converts a block into a control flow graph, this makes easier to compile to LLVM IR, in a
    /// way that we can use the `br` instruction to jump to a block, in the next compiler step.
    ///
    /// E.g.:
    /// ```
    /// if a:
    ///   %1 = (load-argument ...)
    ///   ...
    ///   ret false
    /// else:
    ///   %2 = (load-argument ...)
    ///   ...
    ///   ret true
    /// ```
    /// Will be converted to, using Control Flow Graph form:
    /// ```
    /// bb_1:
    ///   %1 = (load-argument ...)
    ///   ...
    ///   ret false
    ///
    /// bb_2:
    ///   %2 = (load-argument ...)
    ///   ...
    ///   ret true
    ///
    /// entry:
    ///   cond a @bb_1 @bb_2
    /// ```
    pub fn into_control_flow_graph(self) -> BasicBlock<Instruction> {
        // Generates a random id for the basic block
        // to avoid name collisions.
        let id = rand::thread_rng().gen::<u16>();
        let mut bb = BasicBlock::new(&format!("bb_{id}"));
        let instructions = self.block.iter().cloned().flat_map(flatten_instruction);

        for (id, instruction) in instructions.clone().enumerate() {
            match instruction {
                Instruction::Return(value) => {
                    bb.terminator = Terminator::Return(value);
                }
                Instruction::If(if_instruction) => {
                    let then = if_instruction.then.into_control_flow_graph();

                    // use `otherwise` as the remaining code
                    let mut otherwise = if_instruction.otherwise.unwrap_or_default();
                    otherwise.block.extend(instructions.dropping(id + 1));

                    //
                    let otherwise = otherwise.into_control_flow_graph();

                    bb.terminator = Terminator::Cond(
                        if_instruction.condition,
                        Label::new(&then),
                        Label::new(&otherwise),
                    );

                    bb.declared_blocks.push(then);
                    bb.declared_blocks.push(otherwise);
                    break;
                }
                instruction => bb.instructions.push(instruction),
            }
        }
        bb
    }
}

/// Removes all [Instruction::Metadata] from the instruction list, as they are only used
/// for debugging purposes.
///
/// This is done in a later stage, after the code is converted to a control flow graph.
pub fn flatten_instruction(instruction: Instruction) -> Vec<Instruction> {
    match instruction {
        Instruction::If(mut if_instruction) => {
            if_instruction.then.block = if_instruction
                .then
                .block
                .iter()
                .cloned()
                .flat_map(flatten_instruction)
                .collect();

            if let Some(ref mut otherwise) = if_instruction.otherwise {
                otherwise.block = otherwise
                    .block
                    .iter()
                    .cloned()
                    .flat_map(flatten_instruction)
                    .collect();
            }
            vec![Instruction::If(if_instruction)]
        }
        Instruction::Metadata(metadata) => metadata
            .instructions
            .iter()
            .cloned()
            .flat_map(flatten_instruction)
            .collect(),
        instruction => vec![instruction],
    }
}

impl HasTerm for Instruction {
    type Term = Term;
}
