use crate::ir::graph::{BasicBlock, HasTerm};

pub type VisitBlock = BasicBlock<Instruction>;

pub type ArgumentIndex = u64;

/// An internal intermediate representation between HVM <-> LLVM, this is used to
/// generate the visit functions.
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Increases the vlen by the given [ArgumentIndex].
    ///
    /// It only performs the visit, if the argument isn't in the
    /// WHNF form.
    IncreaseLen(ArgumentIndex),

    /// Visits an argument indexed by [ArgumentIndex], reduces it,
    /// and pushes a new visit into the current context.
    ///
    /// It only performs the visit, if current vlen [Instruction::SetVLen], is
    /// less than the given [ArgumentIndex].
    Visit(ArgumentIndex),

    //>>> Internal
    /// Updates the host, by the visit operations, with the given
    /// vbuf, indexed by `vlen - 1`.
    UpdateHost,

    /// Updates the continuation index, by the visit operations, with the given
    /// redex [Term::Redex].
    UpdateCont,

    /// Sets the vbuf in the current environment, by the given [Term::CreateVBuf].
    SetVBuf(Term),

    /// Sets the vlen in the current environment, initialized by 0.
    SetVLen,

    /// Sets a "go up" index, by the given [Term::Redex].
    ///
    /// The "go up" binding in the context, is used to set the location
    /// of the current term, and as a pointer "where" to return, after performing
    /// an [Instruction::Visit] instruction.
    ///
    /// The code only "goes up", when all the strict arguments, are reduced, or in the WHNF
    /// form. This means, that the code only returns true, when all the strict arguments
    /// are reduced, and the current term is in the WHNF form.
    ///
    /// PS: This is only true, if the argument is strict. If the argument is not strict,
    /// then the code won't be reduced in the "visit" stage, and the code will be reduced
    /// in the "apply" stage.
    ///
    /// E.g. The following code, where `Foo` is a constructor, and `Bar` is a function:
    /// ```
    /// (Bar x) = (x)
    ///
    /// (Foo (Bar 10))
    /// ```
    /// Should be reduced to:
    /// ```
    /// (Foo 10)
    /// ```
    /// As the `Bar` function is a function, it's not a constructor, it can be reduced to the
    /// WHNF form, and the `Foo` constructor is already in the WHNF, because it's a constructor.
    SetGoup(Term),
    //<<< Internal
}

#[derive(Debug, Clone)]
pub enum Term {
    /// Creates a new Redex, with the current thread id, in the execution
    /// context, and inserts it in the context with the current thread id.
    ///
    /// It's set by [Instruction::SetGoup].
    Redex,

    //>>> Internal
    /// Gets a new vbuf from the HVM Heap, with the current thread id, in the
    /// execution context.
    ///
    /// It's set in the context by [Instruction::SetVBuf].
    CreateVBuf,

    /// Checks if the vlen is greater than 0, and if it is, it performs
    /// the visit stage.
    CheckVLen,

    True,
    False,
    //<<< Internal
}

impl HasTerm for Instruction {
    type Term = Term;
}
