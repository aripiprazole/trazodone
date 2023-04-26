use inkwell::AddressSpace;

use crate::ir::apply::ApplyBasicBlock;
use crate::ir::rule::RuleGroup;
use crate::llvm::apply::Codegen;

pub type ApplyFn = unsafe extern "C" fn(*mut libc::c_void) -> bool;

impl<'a> Codegen<'a> {
    pub fn build_apply_function(&self, rule: &RuleGroup, _bb: ApplyBasicBlock) -> String {
        // Function signature: <<name>>__apply(%ctx: *mut <<reduce_ctx>>) -> i1
        let function_type = self.context.bool_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()],
            false,
        );

        let name = format!("{}__apply", rule.name);

        let function = self.module.add_function(&name, function_type, None);
        let entry = self.context.append_basic_block(function, "entry");
        let ctx = function.get_first_param().expect("No ctx parameter found");
        ctx.set_name("ctx");
        self.builder.position_at_end(entry);
        self.builder.build_return(Some(&self.context.bool_type().const_int(1, false)));

        // Verify the function integrity
        function.verify(true);

        name
    }
}
