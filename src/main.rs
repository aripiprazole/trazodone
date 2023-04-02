use std::fs::File;

fn main() {
    let example = std::fs::read_to_string("example.hvm").unwrap();
    let file = hvm::language::syntax::read_file(&example).unwrap();
    let rb = hvm::language::rulebook::gen_rulebook(&file);

    println!("Hello, world! {rb:?}");
}

pub struct Codegen {

}