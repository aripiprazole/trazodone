use inkwell::AddressSpace;

use crate::ir::apply::ApplyBasicBlock;
use crate::ir::rule::RuleGroup;
use crate::llvm::apply::Codegen;

pub type ApplyFn = unsafe extern "C" fn(*mut libc::c_void) -> bool;

impl<'a> Codegen<'a> {
    pub fn build_apply_function(&mut self, rule: &RuleGroup, bb: ApplyBasicBlock) -> String {
        // Function signature: <<name>>__apply(%ctx: *mut <<reduce_ctx>>) -> i1
        let function_type = self.context.bool_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()],
            false,
        );

        let name = self.create_mangled_name(rule);
        let function = self.module.add_function(&name, function_type, None);
        let ctx = function.get_first_param().expect("No ctx parameter found");
        ctx.set_name("ctx");

        // Build entry
        println!("DEBUG: {bb}"); // TODO: remove me
        self.ctx = Some(ctx);
        self.build_basic_block(function, bb);

        // Verify the function integrity
        function.verify(true);

        name
    }

    pub fn create_mangled_name(&mut self, rule: &RuleGroup) -> String {
        let hash = format!("{:x}", fxhash::hash64(&rule.name));
        let hash = hash[0..8].to_string();
        format!("_HA{}{}{hash}", rule.name.len(), rule.name)
    }
}
