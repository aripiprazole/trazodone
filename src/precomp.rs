use std::collections::HashMap;
use std::sync::Arc;

use hvm::{Precomp, PrecompFuns, ReduceCtx};

use crate::eval::{Context, Control, Eval};
use crate::ir::rule::RuleGroup;

pub fn compile_eval_precomp(
    precomp: &mut HashMap<u64, Precomp>,
    id: u64,
    smap: &'static [bool],
    group: RuleGroup,
) {
    println!("Precomp {}", group.name);
    println!("{}", group.hvm_visit);
    println!("{:?}", group.hvm_apply);
    println!();

    let name = group.name.clone();
    let hvm_apply = group.hvm_apply;
    let hvm_visit = group.hvm_visit;

    let item = Precomp {
        id,
        name: name.leak(),
        funs: Some(PrecompFuns {
            apply: Arc::new(move |mut ctx| {
                let mut context = Context::new(&mut ctx as *const _ as *mut ReduceCtx);
                let done = hvm_apply.clone().eval(&mut context);
                done.as_bool()
            }),
            visit: Arc::new(move |mut ctx| {
                let mut context = Context::new(&mut ctx as *const _ as *mut ReduceCtx);
                let Control::Break(done) = hvm_visit.clone().eval(&mut context) else {
                    panic!("the program did not finished correctly.")
                };
                done.as_bool()
            }),
        }),
        smap,
    };
    precomp.insert(id, item);
}

pub fn compile_precomp(
    precomp: &mut HashMap<u64, Precomp>,
    id: u64,
    name: &'static str,
    smap: &'static [bool],
) {
    precomp.insert(
        id,
        Precomp {
            id,
            name,
            smap,
            funs: None,
        },
    );
}
