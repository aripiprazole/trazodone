use clap::{Args, Parser, Subcommand};

pub mod eval;
pub mod repl;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug, Clone)]
#[clap(about = "Compile a file and evaluate in JIT or Evaluation mode to Interaction Nets")]
#[clap(aliases = &["exec", "jit", "run", "j", "e", "r"])]
pub struct EvalArgs {
    /// Set the heap size (in 64-bit nodes).
    #[clap(short = 's', long, default_value = "auto", value_parser=parse_size)]
    heap_size: usize,

    /// Set the number of threads to use.
    #[clap(short = 't', long, default_value = "auto", value_parser=parse_thread_ids)]
    thread_ids: usize,

    /// Shows the number of graph rewrites performed.
    #[clap(
        short = 'c',
        long,
        default_value = "false",
        default_missing_value = "true"
    )]
    show_cost: bool,

    /// Toggles debug mode, showing each reduction step.
    #[clap(
        short = 'd',
        long,
        default_value = "false",
        default_missing_value = "true"
    )]
    debug: bool,

    /// Toggles evaluation mode, which uses the evaluation strategy instead of the JIT.
    #[clap(
        short = 'e',
        long,
        default_value = "false",
        default_missing_value = "true"
    )]
    use_eval: bool,

    /// A "file.hvm" to load.
    #[clap(short = 'f', long)]
    file: Option<String>,

    /// The expression to run.
    #[clap(short = 'm', long)]
    main: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Joins the HVM Repl
    #[clap(about = "Joins the HVM Repl")]
    Repl,
    Eval(EvalArgs),
}

pub fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Command::Repl => repl::start_repl(),
        Command::Eval(args) => eval::run_eval(args),
    }
}

fn parse_size(text: &str) -> Result<usize, String> {
    match text {
        "auto" => Ok(hvm::runtime::default_heap_size()),
        _ => text.parse::<usize>().map_err(|x| format!("{}", x)),
    }
}

fn parse_thread_ids(text: &str) -> Result<usize, String> {
    match text {
        "auto" => Ok(hvm::runtime::default_heap_tids()),
        _ => text.parse::<usize>().map_err(|x| format!("{}", x)),
    }
}
