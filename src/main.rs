#![feature(box_patterns)]

use hvm::syntax::Term;

pub mod codegen;
pub mod cstr;
pub mod tree;
pub mod compile;
pub mod phases;
pub mod spec;

fn main() {
    let example = std::fs::read_to_string("example.hvm").unwrap();
    let file = hvm::language::syntax::read_file(&example).unwrap();
    let rb = hvm::language::rulebook::gen_rulebook(&file);
    let mut codegen = codegen::Codegen::new();
    codegen.codegen_main();
    codegen.codegen_rulebook(rb);

    println!("{codegen:?}");
}
