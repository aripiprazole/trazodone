macro_rules! std_llvm_type {
    ($codegen:expr, void) => {
        $codegen.context.void_type()
    };
    ($codegen:expr, ctx) => {
        $codegen
            .context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::default())
    };
    ($codegen:expr, bool) => {
        $codegen.context.bool_type()
    };
    ($codegen:expr, u8) => {
        $codegen.context.i8_type()
    };
    ($codegen:expr, u64) => {
        $codegen.context.i64_type()
    };
}

macro_rules! build_std_functions {
    ($codegen:expr, {$($name:ident($($x:tt),+ $(,)?) -> $ret:tt),+ $(,)?}) => {{
        $({
            let name = stringify!($name);
            let ret = crate::llvm::apply::functions::std_llvm_type!($codegen, $ret);
            let args = &[$(crate::llvm::apply::functions::std_llvm_type!($codegen, $x).into()),+];
            $codegen.module.add_function(name, ret.fn_type(args, false), None);
        })+
    }};
}

macro_rules! std_function {
    ($name:ident(ctx) -> u64) => {
        #[allow(clippy::needless_lifetimes)]
        #[allow(non_snake_case)]
        pub fn $name<'b>(&'b self) -> inkwell::values::BasicValueEnum<'b> {
            self.call_std(stringify!($name), &[])
        }
    };
    ($name:ident(ctx, $($argsn:ident), + $(,)?) -> u64) => {
        #[allow(clippy::needless_lifetimes)]
        #[allow(non_snake_case)]
        pub fn $name<'b>(&'b self, $($argsn: inkwell::values::BasicValueEnum<'b>),+) -> inkwell::values::BasicValueEnum<'b> {
            let arguments = &[$($argsn.into()),+];
            self.call_std(stringify!($name), arguments)
        }
    };
    ($name:ident(ctx) -> void) => {
        #[allow(clippy::needless_lifetimes)]
        #[allow(non_snake_case)]
        pub fn $name<'b>(&'b self) -> inkwell::values::InstructionValue<'b> {
            self.call_void_std(stringify!($name), &[])
        }
    };
    ($name:ident(ctx, $($argsn:ident), + $(,)?) -> void) => {
        #[allow(clippy::needless_lifetimes)]
        #[allow(non_snake_case)]
        pub fn $name<'b>(&'b self, $($argsn: inkwell::values::BasicValueEnum<'b>),+) -> inkwell::values::InstructionValue<'b> {
            let arguments = &[$($argsn.into()),+];
            self.call_void_std(stringify!($name), arguments)
        }
    };
    ($name:ident($($argsn:ident), + $(,)?) -> u64) => {
        #[allow(clippy::needless_lifetimes)]
        #[allow(non_snake_case)]
        pub fn $name<'b>(&'b self, $($argsn: inkwell::values::BasicValueEnum<'b>),+) -> inkwell::values::BasicValueEnum<'b> {
            let arguments = &[$($argsn.into()),+];
            self.call_direct(stringify!($name), arguments)
        }
    };
}

pub(crate) use std_function;
pub(crate) use build_std_functions;
pub(crate) use std_llvm_type;
