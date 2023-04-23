//! Code generation for the *internal* intermediate representation.
//!
//! This internal IR, is used to make easier and maintainable to compile to the
//! final LLVM IR. The internal IR is the imperative representation of the
//! HVM language, and is used to generate the final LLVM IR.
//!
//! The stages of the code generation are:
//!   Apply: This stage generates the code for the `apply` function, which is
//!     the main function of executing a rule. This function is responsible for
//!     executing the rule, and returning the result of the rule. The stages are
//!     the following:
//!       Syntax
//!         -> IR Codegen
//!         -> Inline Declarations
//!         -> Control Flow Graph
//!         -> LLVM IR
//!         -> JIT/AOT // basically executing
//!     TODO: LLVM IR
//!     TODO: Inline Declarations
//!     TODO: Control Flow Graph
//!
//!   Visit: This stage generates the code for the `visit` function, which is
//!     the function that's responsible about reducing to the WHNF form the strict
//!     arguments of a function application.
//!       Syntax
//!         -> IR Codegen
//!         -> LLVM IR
//!         -> JIT/AOT // basically executing
//!     TODO: LLVM IR
//!
//!   Reduce: This stage is the most important stage in terms of performance. This generates
//!     the `FAST APPLY` and `FAST REDUCE` in the original HVM code. This stage is responsible
//!     for generating the state machine to applying and visiting the HVM terms.
//!     TODO: Everything on reduce stage
//!

use std::collections::HashMap;

pub mod apply;
pub mod reduce;
pub mod syntax;
pub mod visit;

impl crate::ir::syntax::RuleGroup {
    pub fn ir_codegen(
        self,
        context: Box<GlobalContext>,
    ) -> apply::Result<crate::ir::rule::RuleGroup> {
        Ok(crate::ir::rule::RuleGroup {
            name: self.name.clone(),
            hvm_visit: visit::Codegen::default().build_visit(&self),
            hvm_apply: apply::Codegen::new(context).build_apply(&self)?,
        })
    }
}

pub fn build_name(name: &str) -> String {
    // TODO: this can still cause some name collisions.
    // Note: avoiding the use of `$` because it is not an actually valid
    // identifier character in C.
    //let name = name.replace('_', "__");
    let name = name.replace('.', "_").replace('$', "_S_");
    format!("_{}_", name)
}

/// The global context is used to generate unique names for the constructors.
#[derive(Debug, Clone)]
pub struct GlobalContext {
    pub name_index: u64,
    /// The constructor name -> id, binding map.
    pub constructors: HashMap<String, u64>,
}

impl Default for GlobalContext {
    fn default() -> Self {
        Self {
            /// The name_index is used to generate unique names for the
            /// constructors.
            ///
            /// The current name index: 29, is defined by the hvm file. Defined
            /// on the HVM github repository.
            name_index: 29, // hvm
            constructors: HashMap::new(),
        }
    }
}
