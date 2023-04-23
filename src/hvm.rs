use std::collections::HashMap;
use std::sync::Arc;

use hvm::rulebook::RuleBook;
use hvm::{Precomp, PrecompFuns, ReduceCtx, PRECOMP};
use itertools::Itertools;

use crate::eval::{Context, Control, Eval};
use crate::ir::rule::RuleGroup;
use crate::llvm::bridge::{initialize_llvm, Bridge};
use crate::llvm::execution::ExecutionEngine;

type StrictMap = &'static [bool];

pub fn setup_precomp(book: RuleBook, groups: HashMap<String, RuleGroup>) {
    let mut precomp = PRECOMP
        .clone()
        .iter()
        .map(|precomp| (precomp.id, precomp.clone()))
        .collect::<HashMap<_, _>>();

    unsafe {
        initialize_llvm();
    }

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
    let hvm_apply = group.hvm_apply.clone();
    let hvm_visit = group.hvm_visit.clone();
    let cfg = hvm_apply.clone().into_control_flow_graph();

    println!("[debug] apply: {:?}", group.name);
    println!("[debug]   ir =");
    println!("{}", cfg);

    unsafe {
        use std::mem::transmute;

        let bridge = Bridge::new(&format!("bridge_eval_{}", name));
        let group = Box::leak(Box::new(group));
        let visit_fn = bridge.create(visit_fn, &format!("{}__visit", name), group);
        let apply_fn = bridge.create(apply_fn, &format!("{}__apply", name), group);
        let execution = ExecutionEngine::try_new(bridge.module).unwrap();

        let _apply_fn: fn(*mut ReduceCtx) -> bool =
            transmute(execution.get_function_address(&apply_fn));

        let _visit_fn: fn(*mut ReduceCtx) -> bool =
            transmute(execution.get_function_address(&visit_fn));

        Precomp {
            id,
            name: name.leak(),
            funs: Some(PrecompFuns {
                apply: Arc::new(move |mut ctx| {
                    // apply_fn(&mut ctx as *const _ as *mut _)
                    let mut context = Context::new(&mut ctx as *const _ as *mut ReduceCtx);
                    let done = hvm_apply.clone().eval(&mut context);
                    done.as_bool()
                }),
                visit: Arc::new(move |mut ctx| {
                    // visit_fn(&mut ctx as *const _ as *mut _)
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
}

fn visit_fn(group: *mut RuleGroup, ctx: *mut ReduceCtx) -> bool {
    unsafe {
        let group = group.read();
        let mut ctx = ctx.read();
        let mut context = Context::new(&mut ctx as *const _ as *mut _);
        let Control::Break(done) = group.hvm_visit.eval(&mut context) else {
            panic!("The program did not finished correctly.")
        };
        done.as_bool()
    }
}

fn apply_fn(group: *mut RuleGroup, ctx: *mut ReduceCtx) -> bool {
    unsafe {
        let group = group.read();
        let mut ctx = ctx.read();
        let mut context = Context::new(&mut ctx as *const _ as *mut _);
        let done = group.hvm_apply.eval(&mut context);
        done.as_bool()
    }
}
