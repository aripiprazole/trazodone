#![feature(box_patterns)]
#![feature(slice_pattern)]
#![feature(string_leak)]
#![feature(get_mut_unchecked)]

extern crate core;

use std::arch::asm;

pub mod cli;
pub mod codegen;
pub mod eval;
pub mod ir;
pub mod llvm;
pub mod hvm;
pub mod runtime;

/// The `trazodone` command entrypoint.
///
/// This project has two main subcommands:
///   - repl
///   - eval
fn main() {
    cli::run_cli();
}
