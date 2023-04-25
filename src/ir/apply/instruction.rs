use crate::ir::apply::{Block, Position, Term};

/// An internal intermediate representation between HVM <-> LLVM, this can be converted
/// to Control Flow Graphs using [crate::codegen::apply::graph], and then to LLVM IR.
///
/// The IR is a list of instructions, each instruction is a [Instruction]. It can have
/// if, switch cases, let bindings, and other instructions. But before compiling to
/// LLVM IR, it must be converted to a Control Flow Graph, and converted to
/// [crate::ir::graph::BasicBlock].
#[derive(Debug, Clone)]
pub enum Instruction {
    Collect(Collect),
    Free(Free),

    /// A metadata instruction, contains the a block of instructions,
    /// the comments and the original term. The documentation is
    /// contained in the [Metadata].
    Metadata(Metadata),

    /// A conditional branch instruction,
    /// if the condition is true, then the then block is executed;
    /// otherwise, the otherwise block is executed.
    If(If),

    /// A binding instruction, binds a name to a value,
    /// the value is evaluated before the binding;
    /// the name is only visible in the block.
    Let(Let),

    /// A link instruction, links the current term to the given term,
    /// the position indicates the position in the HVM heap.
    Link(Link),

    /// An term instruction, this just performs the operation of
    /// the given [Term].
    Term(Term),

    /// An increment cost instruction, increments the cost
    /// of the current term, indicating to the HVM that the
    /// current term is performing a new reduction step.
    IncrementCost,

    /// A println instruction, prints the given string to the standard output.
    /// The string is evaluated before the instruction.
    ///
    /// This instruction is used for debugging purposes.
    Println(String),

    /// A return instruction, returns the given term.
    ///
    /// It's a terminator instruction, for debugging purposes, it's kept
    /// in the instruction list, and removed in a later stage, and replaced
    /// by a [crate::ir::graph::Terminator::Return] terminator.
    Return(Term),
}

/// Represents a metadata instruction, contains the a block of instructions,
/// the comments and the original term.
///
/// This is used to break the code into smaller pieces,
/// and to add comments to the generated IR, and make easier to
/// understand the generated code.
///
/// E.g: This code: `Succ Zero`, generates the following IR:
/// ```
/// %1 = make-agent                           ; term = (Zero)
/// %2 = make-agent (Constructor.new Zero %1) ; term = (Succ (Zero))
/// ```
/// The comments are added to the IR, to make easier to understand.
#[derive(Debug, Clone)]
pub struct Metadata {
    /// The original syntax term.
    pub term: crate::ir::syntax::Term,

    /// The extra comments, to explain something internal,
    /// or to explain the generated code.
    pub comments: Vec<String>,

    /// The block of instructions that are used to
    /// generate the term.
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: Term,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub position: Position,
    pub term: Term,
}

#[derive(Debug, Clone)]
pub struct Collect {
    pub term: Term,
}

#[derive(Debug, Clone)]
pub struct Free {
    pub position: Term,
    pub arity: u64,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Term,
    pub then: Block,
    pub otherwise: Option<Block>,
}

impl Instruction {
    /// Creates a new println instruction.
    pub fn println(name: &str) -> Self {
        Instruction::Println(name.into())
    }

    /// Creates a new binding instruction.
    pub fn binding(name: &str, value: Term) -> Self {
        Instruction::Let(Let {
            name: name.into(),
            value,
        })
    }

    /// Creates a new link instruction.
    pub fn link(position: Position, term: Term) -> Self {
        Instruction::Link(Link { position, term })
    }

    /// Creates a new if instruction.
    pub fn cond(condition: Term, then: Block, otherwise: Option<Block>) -> Self {
        Instruction::If(If {
            condition,
            then,
            otherwise,
        })
    }

    /// Creates a new return instruction.
    pub fn ret(term: Term) -> Self {
        Instruction::Return(term)
    }

    /// Creates a new collect instruction.
    pub fn collect(term: Term) -> Self {
        Instruction::Collect(Collect { term })
    }
}
