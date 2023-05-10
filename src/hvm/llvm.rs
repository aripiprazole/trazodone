use std::sync::Arc;

use fxhash::FxHashMap;
use hvm::rulebook::RuleBook;
use hvm::{Precomp, PrecompFuns, ReduceCtx, PRECOMP};
use inkwell::execution_engine::ExecutionEngine;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::OptimizationLevel;
use itertools::Itertools;

use crate::eval::{Context, Control, Eval};
use crate::ir::rule::RuleGroup;
use crate::llvm::apply::Codegen;

type ApplyFn = unsafe extern "C" fn(*mut libc::c_void) -> bool;

pub fn setup_llvm_precomp(
    book: RuleBook,
    groups: FxHashMap<String, RuleGroup>,
) -> Result<(), String> {
    let mut precomp = PRECOMP
        .clone()
        .iter()
        .map(|precomp| (precomp.id, precomp.clone()))
        .collect::<FxHashMap<_, _>>();

    Target::initialize_native(&InitializationConfig::default())
        .map_err(|e| format!("Could not initialize llvm native target for JIT: {e}"))?;

    for (id, name) in itertools::sorted(book.id_to_name.iter()) {
        let smap = book.id_to_smap.get(id).unwrap().clone().leak();
        if *id <= 29 {
            // Skip built-in constructors
            continue;
        }

        let compiled_function = match groups.get(name) {
            Some(group) => create_llvm_precomp(*id, smap, group.clone()),
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

    Ok(())
}

pub fn create_llvm_precomp(id: u64, smap: &'static [bool], group: RuleGroup) -> Precomp {
    let name = group.name.clone();
    let hvm_apply = group.hvm_apply.clone().into_control_flow_graph();
    let hvm_visit = group.hvm_visit.clone();

    let context = inkwell::context::Context::create();
    let mut codegen = Codegen::new(&context)
        .map_err(|e| format!("Could not create codegen: {e}"))
        .unwrap();
    let engine = codegen
        .module
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| format!("Could not create execution engine: {}", e.to_string_lossy()))
        .unwrap();

    codegen.initialize_std_functions();
    codegen.register_functions_on_jit(&engine);

    let hvm_apply = codegen.build_apply_function(&group, hvm_apply);

    let hvm_apply = engine
        .get_function_address(&hvm_apply)
        .unwrap_or_else(|err| panic!("Could not find function address for {hvm_apply}: {err}",));
    let hvm_apply = unsafe { std::mem::transmute::<_, ApplyFn>(hvm_apply) };

    Precomp {
        id,
        name: name.leak(),
        funs: Some(PrecompFuns {
            apply: Arc::new(move |mut ctx| unsafe {
                let ctx_ref = &mut ctx as *const _ as *mut ReduceCtx;

                hvm_apply(ctx_ref as *mut libc::c_void)
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
