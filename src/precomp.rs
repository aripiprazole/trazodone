use std::collections::HashMap;
use std::sync::Arc;

use hvm::{Precomp, PrecompFuns};

use crate::ir::RuleGroup;
use crate::phases::eval::{Context, Eval};
use crate::phases::spec_ir::{build_name, GlobalContext};

pub fn compile_eval_precomp(
    precomp: &mut Vec<Precomp>,
    id: u64,
    smap: &'static [bool],
    group: RuleGroup,
) {
    let name = group.name.clone();
    precomp.insert(id as usize,Precomp {
        id,
        name: build_name(&name).leak(),
        funs: Some(PrecompFuns {
            apply: Arc::new(move |ctx| unsafe {
                println!("apply: {}", name);
                let mut context = Context {
                    reduce_context: std::mem::transmute(Box::leak(Box::new(ctx))),
                    variables: HashMap::new(),
                };

                let group = group.clone();
                let done = group.hvm_apply.eval(&mut context);
                done.as_bool()
            }),
            visit: Arc::new(move |_| {
                println!("visit");
                // println!("visit: {}", name);
                false
            }),
        }),
        smap,
    });
}

pub fn compile_precomp(
    precomp: &mut Vec<Precomp>,
    id: u64,
    name: &'static str,
    smap: &'static [bool],
) {
    precomp.insert(id as usize, Precomp {
        id,
        name,
        smap,
        funs: None,
    });
}
