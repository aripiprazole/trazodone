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

    let program = book.rule_group.iter().map(|(name, group)| {
        let group = tree::RuleGroup::specialize(name.clone(), &book).unwrap();
        let group = spec::RuleGroup::specialize(group).unwrap();

        group
    }).collect::<Vec<_>>();

    println!("{:#?}", program);
}
