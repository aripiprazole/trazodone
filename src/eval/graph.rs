use std::collections::HashMap;

use crate::eval::{Context, Control, Eval, Object};
use crate::ir::graph::{BasicBlock, HasTerm, Label, Terminator};

impl<E: Eval> Eval for BasicBlock<E>
where
    E: HasTerm,
    E::Term: Eval<Output = Object>,
{
    type Output = Control;

    fn eval(self, context: &mut Context) -> Self::Output {
        let blocks = self
            .declared_blocks
            .iter()
            .map(|bb| (bb.label.clone(), bb.clone()))
            .collect::<HashMap<_, _>>();

        for instruction in self.instructions {
            instruction.eval(context);
        }

        match self.terminator {
            Terminator::Unreachable => {
                panic!("Unreachable");
            }
            Terminator::Debug(message) => {
                println!("{}", message);
                Control::Break(Object::Bool(false))
            }
            Terminator::Return(value) => Control::Break(value.eval(context)),
            Terminator::Jump(Label(label)) => {
                let branch = blocks
                    .get(&label)
                    .expect("could not find then branch")
                    .clone();

                let Control::Break(value) = branch.eval(context) else {
                    panic!("The program did not finished correctly.")
                };

                Control::Break(value)
            }
            Terminator::Cond(cond, Label(then), Label(otherwise)) => {
                let then = blocks
                    .get(&then)
                    .expect("could not find then branch.")
                    .clone();
                let otherwise = blocks
                    .get(&otherwise)
                    .expect("could not find otherwise branch.")
                    .clone();

                if cond.eval(context).as_bool() {
                    let Control::Break(value) = then.eval(context) else {
                        panic!("the program did not finished correctly.")
                    };

                    Control::Break(value)
                } else if let Control::Break(value) = otherwise.eval(context) {
                    Control::Break(value)
                } else {
                    panic!("the program did not finished correctly.")
                }
            }
        }
    }
}
