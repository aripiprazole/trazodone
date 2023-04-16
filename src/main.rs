#![feature(box_patterns)]

use crate::phases::Transform;

pub mod codegen;
pub mod compile;
pub mod cstr;
pub mod ir;
pub mod phases;
pub mod pretty;
pub mod syntax;
pub mod runtime;

fn main() {
    let example = std::fs::read_to_string("example.hvm").unwrap();
    let file = hvm::language::syntax::read_file(&example).unwrap();
    let book = hvm::language::rulebook::gen_rulebook(&file);

    // println!("{:#?}", book.rule_group);

    for group in book.transform().unwrap() {
        println!("=>>>>>>>>");
        println!("Building: {}", group.name);
        println!("{:#?}", group.transform().unwrap());
        println!("<<<<<<<<=");
    }
}
