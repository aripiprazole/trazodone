#![feature(box_patterns)]
#![feature(string_leak)]
#![feature(get_mut_unchecked)]

use std::collections::HashMap;
use std::sync::Arc;

use hvm::{default_precomp, PRECOMP};

use crate::phases::spec_ir::{build_name, GlobalContext};
use crate::phases::Transform;
use crate::precomp::{compile_eval_precomp, compile_precomp};

pub mod codegen;
pub mod compile;
pub mod cstr;
pub mod ir;
pub mod phases;
pub mod precomp;
pub mod pretty;
pub mod runtime;
pub mod syntax;

fn main() {
    let example = std::fs::read_to_string("example.hvm").unwrap();
    let file = hvm::language::syntax::read_file(&example).unwrap();
    let book = hvm::language::rulebook::gen_rulebook(&file);

    // println!("{:#?}", book.rule_group);

    let mut global: Box<GlobalContext> = Box::default();

    for (id, name) in itertools::sorted(book.id_to_name.iter()) {
        global.constructors.insert(build_name(name), *id);
    }

    let groups = book
        .clone()
        .transform()
        .unwrap()
        .iter()
        .map(|group| {
            let name = group.name.clone();
            (name, group.clone().transform_with(global.clone()).unwrap())
        })
        .collect::<HashMap<_, _>>();

    let id_to_smap = book.id_to_smap;
    let id_to_name = book.id_to_name;

    let mut precomp = default_precomp();

    for id in itertools::sorted(id_to_name.keys()) {
        if *id < 30 {
            // skip built-in constructors
            continue;
        }
        let name = id_to_name.get(id).unwrap().clone();
        let smap = id_to_smap.get(id).unwrap().clone();
        let smap = smap.leak();
        match groups.get(&name) {
            Some(group) => {
                compile_eval_precomp(&mut precomp, *id, smap, group.clone());
            }
            None => {
                compile_precomp(&mut precomp, *id, name.clone().leak(), smap);
            }
        }
    }

    unsafe {
        *Arc::get_mut_unchecked(&mut PRECOMP.clone()) = Box::new(precomp);
    }

    run_eval()
}

fn run_eval() {
    let code = std::fs::read_to_string("example.hvm").unwrap();
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
