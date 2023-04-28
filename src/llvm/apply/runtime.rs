use inkwell::module::Linkage;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, InstructionValue};
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

        self.module.add_function(
            "hvm__get_loc",
            self.context.i64_type().fn_type(
                &[
                    self.context.i64_type().into(), // pointer: u64
                    self.context.i64_type().into(), // argument: u64
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__get_ext",
            self.context.i64_type().fn_type(
                &[
                    self.context.i64_type().into(), // pointer: u64
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__get_number",
            self.context.i64_type().fn_type(
                &[
                    self.context.i64_type().into(), // pointer: u64
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__get_tag",
            self.context.i64_type().fn_type(
                &[
                    self.context.i64_type().into(), // pointer: u64
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__alloc",
            self.context.i64_type().fn_type(
                &[
                    self.context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .into(),
                    self.context.i64_type().into(), // arity: u64
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__llvm_eq",
            self.context.bool_type().fn_type(
                &[
                    self.context.i64_type().into(), // a: u64
                    self.context.i64_type().into(), // b: u64
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__llvm_or",
            self.context.bool_type().fn_type(
                &[
                    self.context.bool_type().into(), // a: i1
                    self.context.bool_type().into(), // b: i1
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__llvm_and",
            self.context.bool_type().fn_type(
                &[
                    self.context.bool_type().into(), // a: i1
                    self.context.bool_type().into(), // b: i1
                ],
                false,
            ),
            Some(Linkage::External),
        );

        self.module.add_function(
            "hvm__get_ext",
            self.context.i64_type().fn_type(
                &[
                    self.context.i64_type().into(), // pointer: u64
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
            .unwrap_or_else(|| panic!("{} should return a BasicValueEnum", name))
    }

    pub fn call_direct<'b>(
        &'b self,
        name: &str,
        args: &[BasicMetadataValueEnum<'b>],
    ) -> BasicValueEnum<'b> {
        self.builder
            .build_direct_call(self.module.get_function(name).unwrap(), args.as_ref(), "")
            .try_as_basic_value()
            .left()
            .unwrap_or_else(|| panic!("{} should return a BasicValueEnum", name))
    }

    pub fn call_void_std<'b>(
        &'b self,
        name: &str,
        args: &[BasicMetadataValueEnum<'b>],
    ) -> InstructionValue<'b> {
        let mut complete_args: Vec<BasicMetadataValueEnum> = vec![self.ctx.unwrap().into()];
        complete_args.extend_from_slice(args);

        self.builder
            .build_direct_call(
                self.module.get_function(name).unwrap(),
                complete_args.as_ref(),
                "",
            )
            .try_as_basic_value()
            .right()
            .unwrap_or_else(|| panic!("{} should return an InstructionValue", name))
    }
}
