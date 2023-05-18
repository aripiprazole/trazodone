# trazodone

The Trazodone project, is a Higher-Order Virtual Machine (HVM) backend for LLVM provides a powerful and efficient way to
generate code for virtual machines. It leverages the LLVM infrastructure to optimize and generate high-quality code from
a high-level representation. It's a cutting-edge component that enables the generation of optimized code for virtual
machines. It seamlessly integrates into the LLVM compiler infrastructure, leveraging its robust optimization and code
generation capabilities.

At its core, the HVM backend operates on a high-level representation of code, allowing developers to express complex
virtual machine instructions and operations in a concise and intuitive manner. This representation serves as an
abstraction layer, shielding developers from low-level details while enabling them to write efficient and expressive
code.

The HVM backend takes advantage of LLVM's powerful optimization passes, such as inlining, loop unrolling, and constant
propagation, to analyze and transform the high-level representation into highly optimized code. This optimization
process improves performance, reduces memory usage, and enhances overall execution efficiency.

Moreover, Trazodone incorporates sophisticated just-in-time (JIT) compilation techniques. This means that virtual
machine code can be dynamically compiled and optimized at runtime, leading to improved performance and adaptability. JIT
compilation enables efficient execution of dynamically generated code, such as just-in-time bytecode compilers or
dynamic language interpreters.

## Command-Line Interface

The current command-line interface have the `repl` and `eval` function, but the `repl` currently, doesn't run :/

```
Usage: trazodone <COMMAND>

Commands:
  repl  Joins the HVM Repl
  eval  Compile a file and evaluate in JIT or Evaluation mode to Interaction Nets
  help  Print this message or the help of the given subcommand(s)

Options:
```

To eval a file, it's simple to use, just run the following command:

```bash
$ trazodone eval -f example.brex
```

And use 

```bash
$ trazodone eval -f example.brex --use-eval
```

To evaluate the program without using the LLVM stuff.

The eval help menu, is the following:

```
Compile a file and evaluate in JIT or Evaluation mode to Interaction Nets

Usage: trazodone eval [OPTIONS]

Options:
  -s, --heap-size <HEAP_SIZE>    Set the heap size (in 64-bit nodes) [default: auto]
  -t, --thread-ids <THREAD_IDS>  Set the number of threads to use [default: auto]
  -c, --show-cost                Shows the number of graph rewrites performed
  -d, --debug                    Toggles debug mode, showing each reduction step
  -e, --use-eval                 Toggles evaluation mode, which uses the evaluation strategy instead of the JIT
  -f, --file <FILE>              A "file.hvm" to load
  -m, --main <MAIN>              The expression to run
  -h, --help                     Print help
  -V, --version                  Print version
```

## Features

- [x] Control-Flow Graph representation
- [x] Different trees between Control-Flow Graph and Imperative code
- [x] Codegen for LLVM
- [x] Simple command-line interface
- [x] Evaluator
  It's meant to be used when testing the Intermediate Representation.
- [ ] Super position and duplication
- [ ] Inlining operations like `U60.if`
- [ ] Transmutation optimization
- [ ] Some optimizations in `alloc` and reusing code
- [ ] JIT interpreter
  Currently have a segfault issue
- [ ] AOT Compiler
  - [ ] Split the Runtime code of the HVM into `runtime`, `cli` and `compiler`
    The `runtime` crate, should be linked with the target binary

## Future objectives

- [x] Creating an Intermediate Representation
  Creating an Intermediate Representation is really important for generating code, as it's structure simplifies the code
  into a code that's an intermediate between Turing Machines and Interaction Nets.
- [ ] REPL for HVM
- [ ] Unit testing
- [ ] Bridge between Rust and LLVM to test eval without having to use an `Arc`
- [ ] Stop using `PRECOMP` const from HVM project
  The `PRECOMP` is a global constant that holds all compiled stuff with HVM, but it's not suitable for using it with JIT
- [ ] Compiling to LLVM properly
    -  [x] Compile `apply` function code
    -  [ ] Compile `visit` function code
    -  [ ] Compile `reducer` function code
