#![feature(box_patterns)]
#![feature(string_leak)]
#![feature(get_mut_unchecked)]

pub mod cli;
pub mod codegen;
pub mod cstr;
pub mod eval;
pub mod ir;
pub mod precomp;
pub mod pretty;
pub mod runtime;
pub mod spec;
pub mod syntax;

fn main() {
    cli::run_cli();
}
