#![feature(box_patterns)]
#![feature(slice_pattern)]
#![feature(string_leak)]
#![feature(get_mut_unchecked)]

extern crate core;

pub mod cli;
pub mod codegen;
pub mod eval;
pub mod hvm;
pub mod ir;
pub mod llvm;
pub mod runtime;

/// The `trazodone` command entrypoint.
///
/// This project has two main subcommands:
///   - repl
///   - eval
fn main() {
    cli::run_cli();
}
