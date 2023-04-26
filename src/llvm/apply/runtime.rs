use inkwell::module::Linkage;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};
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

    pub fn call_std<'b>(
        &'b self,
        name: &str,
        args: &[BasicMetadataValueEnum<'b>],
    ) -> BasicValueEnum<'b> {
        let mut complete_args: Vec<BasicMetadataValueEnum> = vec![self.ctx.unwrap().into()];
        complete_args.extend_from_slice(args);

        self.builder
            .build_direct_call(
                self.module.get_function(name).unwrap(),
                complete_args.as_ref(),
                "",
            )
            .try_as_basic_value()
            .left()
            .unwrap()
    }
}
