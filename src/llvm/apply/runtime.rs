use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, InstructionValue};

use crate::llvm::apply::functions::{build_std_functions, std_function};
use crate::llvm::apply::Codegen;

impl<'a> Codegen<'a> {
    pub fn initialize_std_functions(&self) {
        build_std_functions!(self, {
            // create functions
            hvm__create_function(u64, u64) -> u64,
            hvm__create_constructor(u64, u64) -> u64,
            hvm__create_app(u64) -> u64,
            hvm__create_var(u64) -> u64,
            hvm__create_lam(u64) -> u64,
            hvm__create_f60(u64) -> u64,
            hvm__create_u60(u64) -> u64,
            hvm__create_binary(u64, u64) -> u64,
            hvm__create_erased() -> u64,

            // std functions
            hvm__get_host_value(ctx) -> u64,
            hvm__increment_cost(ctx) -> void,
            hvm__get_term(ctx) -> u64,
            hvm__link(ctx, u64, u64) -> u64,
            hvm__load_argument(ctx, u64, u64) -> u64,
            hvm__get_loc(u64, u64) -> u64,
            hvm__get_ext(u64) -> u64,
            hvm__get_number(u64) -> u64,
            hvm__get_tag(u64) -> u64,
            hvm__alloc(ctx, u64) -> u64,
            hvm__llvm_eq(u64, u64) -> bool,
            hvm__llvm_or(bool, bool) -> bool,
            hvm__llvm_and(bool, bool) -> bool,

            // instructions
            hvm__free(ctx, u64, u64) -> void,
        });
    }

    std_function! { hvm__create_function(fn_id, ptr) -> u64 }
    std_function! { hvm__create_constructor(fn_id, ptr) -> u64 }
    std_function! { hvm__create_app(position) -> u64 }
    std_function! { hvm__create_lam(position) -> u64 }
    std_function! { hvm__create_var(position) -> u64 }
    std_function! { hvm__create_f60(value) -> u64 }
    std_function! { hvm__create_u60(value) -> u64 }
    std_function! { hvm__create_binary(operand, position) -> u64 }
    std_function! { hvm__create_erased() -> u64 }

    std_function! { hvm__get_host_value(ctx) -> u64 }
    std_function! { hvm__increment_cost(ctx) -> void }
    std_function! { hvm__get_term(ctx) -> u64 }
    std_function! { hvm__load_argument(ctx, a, b) -> u64 }
    std_function! { hvm__link(ctx, position, ptr) -> u64 }
    std_function! { hvm__get_loc(a, b) -> u64 }
    std_function! { hvm__get_ext(a) -> u64 }
    std_function! { hvm__get_number(a) -> u64 }
    std_function! { hvm__get_tag(a) -> u64 }
    std_function! { hvm__alloc(ctx, a) -> u64 }
    std_function! { hvm__llvm_eq(a, b) -> u64 }
    std_function! { hvm__llvm_or(a, b) -> u64 }
    std_function! { hvm__llvm_and(a, b) -> u64 }

    std_function! { hvm__free(ctx, position, arity) -> void }

    pub fn u64(&self, value: u64) -> BasicValueEnum {
        self.context.i64_type().const_int(value, false).into()
    }

    pub fn f64(&self, value: f64) -> BasicValueEnum {
        self.context.f64_type().const_float(value).into()
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
