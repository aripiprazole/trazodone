use clap::error::ErrorKind;
use clap::CommandFactory;
use colored::Colorize;
use fxhash::FxHashMap;
use hvm::rulebook::RuleBook;
use ErrorKind::InvalidValue;

use crate::cli::{Cli, EvalArgs};
use crate::codegen::syntax::Transform;
use crate::codegen::GlobalContext;
use crate::ir::rule::RuleGroup;

pub fn run_eval(args: EvalArgs) {
    let mut cli = Cli::command();

    let heap_size = args.heap_size;
    let thread_ids = args.thread_ids;
    let debug = args.debug;

    let code = args.file.unwrap_or_else(|| {
        cli.error(InvalidValue, "No expression or file provided!")
            .exit();
    });
    let code = std::fs::read_to_string(code).unwrap_or_else(|_| {
        cli.error(InvalidValue, "Failed to read file.").exit();
    });
    let main = args.main.clone().unwrap_or("Main".into());

    setup_eval_environment(&code);

    let native_functions = Vec::new();
    let (norm, cost, time) =
        hvm::api::eval(&code, &main, native_functions, heap_size, thread_ids, debug)
            .unwrap_or_else(|err| {
                eprintln!("Failed to eval: {}", code);
                if args.debug {
                    panic!("{err}");
                } else {
                    cli.error(InvalidValue, "To get the backtrace, run with --debug or -d")
                        .exit();
                }
            });

    println!("{norm}");

    if args.show_cost {
        let time = (time as f64) / 1000.0;
        let total_cost = cost - 1;
        let rps = (cost as f64) / time / 1000.0;
        let cost_msg = format!("[TIME: {time:.2}s | COST: {total_cost} | RPS: {rps:.2}m]");

        println!("{}", cost_msg.bright_blue())
    }
}

fn setup_eval_environment(code: &str) {
    let mut cli = Cli::command();
    let file = match hvm::language::syntax::read_file(code) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to parse: {}", code);
            eprintln!("{}", err);
            cli.error(ErrorKind::InvalidValue, "Failed to parse file.")
                .exit();
        }
    };
    let book = hvm::language::rulebook::gen_rulebook(&file);

    let global = setup_global_context(&book);
    let groups = ir_codegen_book(&book, global);

    crate::hvm::setup_precomp(book, groups);
}

fn setup_global_context(book: &RuleBook) -> Box<GlobalContext> {
    let mut id_to_name = book.id_to_name.clone();
    id_to_name.remove(book.name_to_id.get("Main").unwrap());

    let mut global: Box<GlobalContext> = Box::default();
    for (id, name) in itertools::sorted(id_to_name.iter()) {
        global.constructors.insert(name.clone(), *id);
    }

    global
}

fn ir_codegen_book(book: &RuleBook, global: Box<GlobalContext>) -> FxHashMap<String, RuleGroup> {
    book.clone()
        .transform()
        .unwrap()
        .iter()
        .map(|group| {
            let name = group.name.clone();
            (name, group.clone().ir_codegen(global.clone()).unwrap())
        })
        .collect()
}
