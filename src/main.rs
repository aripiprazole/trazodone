#![feature(box_patterns)]

pub mod codegen;
pub mod cstr;
pub mod tree;
pub mod compile;
pub mod spec;
pub mod pretty;

fn main() {
    let example = std::fs::read_to_string("example.hvm").unwrap();
    let file = hvm::language::syntax::read_file(&example).unwrap();
    let book = hvm::language::rulebook::gen_rulebook(&file);

    let group = tree::RuleGroup::specialize("SwapGT".into(), book).unwrap();
    let group = spec::RuleGroup::specialize(group).unwrap();

    println!("{:#?}", group);
}
