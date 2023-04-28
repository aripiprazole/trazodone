use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, InstructionValue};

use crate::llvm::apply::functions::std_function;
use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    pub fn initialize_std_functions(&self) {
        std_function!(self, {
            hvm__increment_cost(ctx) -> void,
            hvm__get_term(ctx) -> u64,
            hvm__load_argument(ctx, u64, u64) -> u64,
            hvm__get_loc(u64, u64) -> u64,
            hvm__get_ext(u64) -> u64,
            hvm__get_number(u64) -> u64,
            hvm__get_tag(u64) -> u64,
            hvm__alloc(ctx, u64) -> u64,
            hvm__llvm_eq(u64, u64) -> bool,
            hvm__llvm_or(bool, bool) -> bool,
            hvm__llvm_and(bool, bool) -> bool,
        });
    }

    /// Call a function from the HVM runtime, that passes the context as the first argument.
    /// This is used for functions that are not part of the HIR, but are part of the runtime.
    ///
    /// # Example
    /// ```
    /// let term = self.build_term(*get_ext.term);
    ///
    /// self.call_std("hvm__get_ext", &[term.into()])
    /// ```
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

    /// Call a function from the HVM runtime, that passes the context as the first argument, and
    /// returns nothing(or void, or unit).
    /// This is used for functions that are not part of the HIR, but are part of the runtime.
    ///
    /// # Example
    /// ```
    /// self.call_void_std("hvm__increment_cost", &[])
    /// ```
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

    /// Call a function a function that returns a [BasicValueEnum].
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
}
