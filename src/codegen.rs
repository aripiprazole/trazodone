use std::collections::HashMap;

pub mod apply;
pub mod reduce;
pub mod visit;

impl crate::syntax::RuleGroup {
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

#[derive(Debug, Clone)]
pub struct GlobalContext {
    pub name_index: u64,
    pub constructors: HashMap<String, u64>,
}

impl Default for GlobalContext {
    fn default() -> Self {
        Self {
            name_index: 29, // precomp.rs
            constructors: HashMap::new(),
        }
    }
}
