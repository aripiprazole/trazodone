#![feature(box_patterns)]
#![feature(string_leak)]
#![feature(get_mut_unchecked)]

pub mod cli;
pub mod codegen;
pub mod eval;
pub mod ir;
pub mod precomp;
pub mod runtime;
pub mod llvm;

fn main() {
    cli::run_cli();
}
