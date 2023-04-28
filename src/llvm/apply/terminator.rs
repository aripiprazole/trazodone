use inkwell::values::FunctionValue;

use crate::ir::{
    apply::ApplyBasicBlock,
    graph::{Label, Terminator},
};

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn build_terminator(&mut self, bb: ApplyBasicBlock, function: FunctionValue<'a>) {
        match bb.terminator {
            Terminator::Unreachable => {}
            Terminator::Debug(_) => {}
            Terminator::Jump(_) => {}
            Terminator::Return(value) => {
                self.builder.build_return(Some(&self.build_term(value)));
            }
            Terminator::Cond(cond, Label(then), Label(otherwise)) => {
                let old_bb = self.bb.unwrap();

                let then = bb.declared_blocks.get(&then).unwrap();
                let then = self.build_basic_block(function, then.clone());
                let otherwise = bb.declared_blocks.get(&otherwise).unwrap();
                let otherwise = self.build_basic_block(function, otherwise.clone());

                self.builder.position_at_end(old_bb);
                self.builder.build_conditional_branch(
                    self.build_term(cond).into_int_value(),
                    then,
                    otherwise,
                );
            }
        }
    }
}
