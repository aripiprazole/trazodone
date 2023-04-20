use std::sync::atomic::AtomicU64;

use crate::eval::{Context, Eval, Object};
use crate::ir::visit::{Instruction, Term};
use crate::runtime::{hvm__create_vbuf, hvm__increase_vlen, hvm__insert_redex, hvm__new_redex, hvm__update_cont, hvm__update_host, hvm__visit};

impl Eval for Instruction {
    type Output = ();

    fn eval(self, context: &mut Context) -> Self::Output {
        unsafe {
            match self {
                Instruction::SetVLen => {
                    context.variables.insert("vlen".into(), Object::U64(0));
                }
                Instruction::SetVBuf(vbuf) => {
                    let vbuf = vbuf.eval(context);
                    context.variables.insert("vbuf".into(), vbuf);
                }
                Instruction::SetGoup(redex) => {
                    let goup = redex.eval(context);
                    context.variables.insert("goup".into(), goup);
                }
                Instruction::UpdateCont => {
                    let variables = &mut context.variables;
                    let vlen = variables.get("vlen").expect("vlen not found").as_u64();

                    hvm__update_cont(context.reduce, vlen);
                }
                Instruction::UpdateHost => {
                    let variables = &mut context.variables;
                    let vbuf = variables
                        .get("vbuf")
                        .expect("vbuf not found")
                        .as_ptr::<Box<[AtomicU64]>>();
                    let vlen = variables.get("vlen").expect("vlen not found").as_u64();

                    hvm__update_host(context.reduce, vbuf, vlen);
                }
                Instruction::IncreaseLen(parameter_index) => {
                    let variables = &mut context.variables;

                    let vbuf = variables
                        .get("vbuf")
                        .expect("vbuf not found")
                        .as_ptr::<Box<[AtomicU64]>>();

                    let vlen = variables.get("vlen").expect("vlen not found").as_u64();

                    let new_vlen = hvm__increase_vlen(context.reduce, parameter_index, vbuf, vlen);

                    variables.insert("vlen".into(), Object::U64(vlen + new_vlen));
                }
                Instruction::Visit(parameter_index) => {
                    let variables = &mut context.variables;

                    let vbuf = variables
                        .get("vbuf")
                        .expect("vbuf not found")
                        .as_ptr::<Box<[AtomicU64]>>();

                    let vlen = variables.get("vlen").expect("vlen not found").as_u64();
                    let goup = variables.get("goup").expect("goup not found").as_u64();

                    hvm__visit(context.reduce, parameter_index, goup, vbuf, vlen);
                }
            }
        }
    }
}

impl Eval for Term {
    type Output = Object;

    fn eval(self, context: &mut Context) -> Self::Output {
        unsafe {
            match self {
                Term::True => Object::Bool(true),
                Term::False => Object::Bool(false),
                Term::CreateVBuf => {
                    Object::Pointer(std::mem::transmute(hvm__create_vbuf(context.reduce)))
                }
                Term::Redex => {
                    let vlen = context.variables.get("vlen").expect("vlen not found").as_u64();
                    let redex = hvm__new_redex(context.reduce, vlen);

                    Object::U64(hvm__insert_redex(context.reduce, redex))
                }
                Term::CheckVLen => {
                    let vlen = context.variables.get("vlen").expect("vlen not found").as_u64();

                    Object::Bool(vlen != 0)
                }
            }
        }
    }
}
