use itertools::Itertools;
use crate::ir::graph::BasicBlock;
use crate::ir::visit::{Instruction, Term};
use crate::syntax::RuleGroup;

#[derive(Default, Debug, Clone)]
pub struct Codegen {
    pub basic_blocks: Vec<BasicBlock<Instruction>>,
    pub instructions: BasicBlock<Instruction>,
}

impl Codegen {
    pub fn build_visit(&mut self, group: &RuleGroup) -> BasicBlock<Instruction> {
        let mut bb = self.new_block("entry", move |this, bb| {
            bb.with_return(Term::False);

            if !group.strict_parameters.is_empty() {
                bb.instructions.push(Instruction::SetVLen);
                bb.instructions.push(Instruction::SetVBuf(Term::CreateVBuf));

                for (index, is_strict) in group.strict_parameters.iter().enumerate() {
                    if !is_strict {
                        continue;
                    }

                    bb.instructions.push(Instruction::IncreaseLen(index as u64));
                }

                let goup_bb = this.new_block("goup", |_, bb| {
                    bb.instructions.push(Instruction::SetGoup(Term::Redex));
                    for (index, is_strict) in group.strict_parameters.iter().enumerate() {
                        if !is_strict {
                            continue;
                        }

                        bb.instructions.push(Instruction::Visit(index as u64));
                    }
                    bb.instructions.push(Instruction::UpdateCont);
                    bb.instructions.push(Instruction::UpdateHost);
                    bb.with_return(Term::True);
                });

                let return_bb = this.new_block("otherwise", |_, bb| {
                    bb.with_return(Term::False)
                });

                bb.with_cond(Term::CheckVLen, &goup_bb, &return_bb);
            }
        });
        bb.declared_blocks = self.basic_blocks.iter().dropping_back(1).cloned().collect();
        bb
    }

    pub fn new_block<F>(&mut self, name: &str, f: F) -> BasicBlock<Instruction>
    where
        F: FnOnce(&mut Self, &mut BasicBlock<Instruction>),
    {
        let mut bb = BasicBlock::new(name);
        f(self, &mut bb);
        self.basic_blocks.push(bb);
        self.basic_blocks.last().unwrap().clone()
    }
}
