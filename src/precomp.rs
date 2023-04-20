use std::collections::HashMap;
use std::sync::Arc;

use hvm::{Precomp, PRECOMP, PrecompFuns, ReduceCtx};
use hvm::rulebook::RuleBook;
use itertools::Itertools;

use crate::eval::{Context, Control, Eval};
use crate::ir::rule::RuleGroup;

pub fn setup_precomp(book: RuleBook, groups: HashMap<String, RuleGroup>) {
    let mut precomp = PRECOMP
        .clone()
        .iter()
        .map(|precomp| (precomp.id, precomp.clone()))
        .collect::<HashMap<_, _>>();

    for (id, name) in itertools::sorted(book.id_to_name.iter()) {
        let smap = book.id_to_smap.get(id).unwrap().clone().leak();
        if *id <= 29 {
            // skip built-in constructors
            continue;
        }

        match groups.get(name) {
            Some(group) => {
                compile_eval_precomp(&mut precomp, *id, smap, group.clone());
            }
            None => {
                compile_precomp(&mut precomp, *id, name.clone().leak(), smap);
            }
        }
    }

    unsafe {
        let reordered = precomp
            .iter()
            .sorted_by_key(|(id, _)| *id)
            .map(|(_, precomp)| precomp.clone())
            .collect::<Vec<_>>();

        *Arc::get_mut_unchecked(&mut PRECOMP.clone()) = Box::new(reordered);
    }
}

pub fn compile_eval_precomp(
    precomp: &mut HashMap<u64, Precomp>,
    id: u64,
    smap: &'static [bool],
    group: RuleGroup,
) {
    let name = group.name.clone();
    let hvm_apply = group.hvm_apply;
    let hvm_visit = group.hvm_visit;

    let item = Precomp {
        id,
        name: name.leak(),
        funs: Some(PrecompFuns {
            apply: Arc::new(move |mut ctx| {
                println!("[debug] apply: {:?}", group.name.clone());
                println!("[debug]   ir =");
                println!("{:#?}", hvm_apply.clone());
                let mut context = Context::new(&mut ctx as *const _ as *mut ReduceCtx);
                let done = hvm_apply.clone().eval(&mut context);
                println!("[debug]   apply = {:?}", done);
                done.as_bool()
            }),
            visit: Arc::new(move |mut ctx| {
                // FIXME: its broken :c
                return false;
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
