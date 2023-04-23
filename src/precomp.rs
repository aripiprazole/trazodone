use std::collections::HashMap;
use std::sync::Arc;

use hvm::rulebook::RuleBook;
use hvm::{Precomp, PrecompFuns, ReduceCtx, PRECOMP};
use itertools::Itertools;

use crate::eval::{Context, Control, Eval};
use crate::ir::rule::RuleGroup;

type StrictMap = &'static [bool];

pub fn setup_precomp(book: RuleBook, groups: HashMap<String, RuleGroup>) {
    let mut precomp = PRECOMP
        .clone()
        .iter()
        .map(|precomp| (precomp.id, precomp.clone()))
        .collect::<HashMap<_, _>>();

    for (id, name) in itertools::sorted(book.id_to_name.iter()) {
        let smap = book.id_to_smap.get(id).unwrap().clone().leak();
        if *id <= 29 {
            // Skip built-in constructors
            continue;
        }

        let compiled_function = match groups.get(name) {
            Some(group) => create_precomp(*id, smap, group.clone()),
            // Interpreted function
            None => Precomp {
                id: *id,
                name: name.clone().leak(),
                smap,
                funs: None,
            },
        };

        precomp.insert(*id, compiled_function);
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

pub fn create_precomp(id: u64, smap: StrictMap, group: RuleGroup) -> Precomp {
    let name = group.name.clone();
    let hvm_apply = group.hvm_apply;
    let hvm_visit = group.hvm_visit;

    println!("[debug] apply: {:?}", group.name);
    println!("[debug]   ir =");
    println!("{:#?}", hvm_apply);

    Precomp {
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
    }
}
