use std::collections::HashMap;
use std::sync::Arc;

use hvm::{Precomp, PrecompFuns, ReduceCtx};

use crate::ir::apply::RuleGroup;
use crate::phases::eval::{Context, Eval};

pub fn compile_eval_precomp(
    precomp: &mut HashMap<u64, Precomp>,
    id: u64,
    smap: &'static [bool],
    group: RuleGroup,
) {
    println!("{:?}", group.hvm_apply);
    let name = group.name.clone();
    let xname = group.name.clone();
    precomp.insert(
        id,
        Precomp {
            id,
            name: name.clone().leak(),
            funs: Some(PrecompFuns {
                apply: Arc::new(move |mut ctx| {
                    println!("ir: {}", name.clone());
                    let ctx = &mut ctx;
                    let ctx = ctx as *const _ as *mut ReduceCtx;
                    let mut context = Context {
                        reduce: ctx,
                        variables: HashMap::new(),
                    };

                    let group = group.clone();
                    let done = group.hvm_apply.eval(&mut context);
                    done.as_bool()
                }),
                visit: Arc::new(move |_| {
                    println!("visit: {}", xname);

                    false
                }),
            }),
            smap,
        },
    );
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
