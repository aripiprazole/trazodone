use inkwell::module::Linkage;
use inkwell::AddressSpace;

use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    pub fn initialize_std_functions(&self) {
        self.module.add_function(
            "hvm__increment_cost",
            self.context.void_type().fn_type(
                &[self
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into()],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__get_term",
            self.context.i64_type().fn_type(
                &[self
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into()],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__load_argument",
            self.context.i64_type().fn_type(
                &[
                    self.context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .into(),
                    self.context.i64_type().into(), // term: u64
                    self.context.i64_type().into(), // index: u64
                ],
                false,
            ),
            Some(Linkage::External),
        );
    }
}
