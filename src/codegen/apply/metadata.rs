use crate::codegen::apply::Codegen;
use crate::ir::apply::{Block, Instruction, Metadata, Term};
use crate::ir::syntax::Term as IRTerm;

impl Codegen {
    /// Generates an [Instruction::Metadata] instruction, with
    /// the given [IRTerm] and the given builder function [F].
    ///
    /// The builder function [F] receives a new [Codegen] instance without
    /// any [Instruction]s, and the given [IRTerm].
    pub fn with_metadata<F>(&mut self, current: IRTerm, f: F) -> Term
    where
        F: FnOnce(&mut Codegen, IRTerm) -> Term,
    {
        let mut metadata = Metadata {
            term: current.clone(),
            comments: Vec::new(),
            instructions: Vec::new(),
        };
        let mut codegen = self.new_empty_block();
        let new_term = f(&mut codegen, current);
        metadata.instructions = codegen.instructions.block;
        // Update the current name index, with the used name index
        // to maintain the name index consistency
        self.name_index = codegen.name_index;

        self.instr(Instruction::Metadata(metadata));

        new_term
    }

    fn new_empty_block(&self) -> Self {
        Self {
            global: self.global.clone(),
            name_index: self.name_index,
            variables: self.variables.clone(),
            lambdas: self.lambdas.clone(),
            instructions: Block::default(),
            // constant clonning
            constant_extensions: self.constant_extensions.clone(),
            constant_tags: self.constant_tags.clone(),
        }
    }
}
