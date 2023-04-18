#![feature(box_patterns)]
#![feature(string_leak)]
#![feature(get_mut_unchecked)]

use std::collections::HashMap;
use std::sync::Arc;

use hvm::rulebook::RuleBook;
use hvm::PRECOMP;
use itertools::Itertools;

use crate::codegen::{build_name, GlobalContext};
use crate::ir::rule::RuleGroup;
use crate::spec::Transform;
use crate::precomp::{compile_eval_precomp, compile_precomp};

pub mod codegen;
pub mod cstr;
pub mod ir;
pub mod spec;
pub mod precomp;
pub mod pretty;
pub mod runtime;
pub mod syntax;
pub mod eval;

fn main() {
    let code = std::fs::read_to_string("example.hvm").unwrap();
    let file = hvm::language::syntax::read_file(&code).unwrap();
    let book = hvm::language::rulebook::gen_rulebook(&file);
    let mut id_to_name = book.id_to_name.clone();
    id_to_name.remove(book.name_to_id.get("Main").unwrap());

    let mut global: Box<GlobalContext> = Box::default();
    for (id, name) in itertools::sorted(id_to_name.iter()) {
        global.constructors.insert(build_name(name), *id);
    }

    let groups = book
        .clone()
        .transform()
        .unwrap()
        .iter()
        .map(|group| {
            let name = group.name.clone();
            (name, group.clone().ir_codegen(global.clone()).unwrap())
        })
        .collect::<HashMap<_, _>>();

    setup_precomp(book, groups);
    run_eval(code)
}

fn setup_precomp(book: RuleBook, groups: HashMap<String, RuleGroup>) {
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

fn run_eval(code: String) {
    let (norm, cost, time) = hvm::api::eval(
        &code,
        "Main",
        Vec::new(),
        hvm::default_heap_size(),
        hvm::default_heap_tids(),
        false,
    )
    .unwrap();
    println!("{}", norm);
    eprintln!();
    eprintln!(
        "\x1b[32m[TIME: {:.2}s | COST: {} | RPS: {:.2}m]\x1b[0m",
        ((time as f64) / 1000.0),
        cost - 1,
        (cost as f64) / (time as f64) / 1000.0
    );
}
