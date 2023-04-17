use crate::ir::graph::{BasicBlock, Terminator};
use crate::ir::visit::{Instruction, Term};
use crate::syntax::RuleGroup;

#[derive(Default, Debug, Clone)]
pub struct Codegen {
    pub instructions: BasicBlock<Instruction>,
}

impl Codegen {
    pub fn build_visit(&mut self, group: &RuleGroup) -> BasicBlock<Instruction> {
        let mut bb = BasicBlock::new("entry");
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

            let mut goup_bb = BasicBlock::<Instruction>::new("goup");
            goup_bb.instructions.push(Instruction::SetGoup(Term::Redex));
            for (index, is_strict) in group.strict_parameters.iter().enumerate() {
                if !is_strict {
                    continue;
                }

                goup_bb.instructions.push(Instruction::Visit(index as u64));
            }
            goup_bb.instructions.push(Instruction::SetCont);
            goup_bb.instructions.push(Instruction::SetHost);
            goup_bb.with_return(Term::True);

            let mut return_bb = BasicBlock::<Instruction>::new("otherwise");
            return_bb.with_return(Term::False);

            bb.with_cond(Term::CheckVLen, goup_bb, return_bb);
        }

        todo!()
    }
}
