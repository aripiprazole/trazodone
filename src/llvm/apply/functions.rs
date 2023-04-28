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

macro_rules! std_function {
    ($codegen:expr, {$($name:ident($($x:tt),+ $(,)?) -> $ret:tt),+ $(,)?}) => {{
        $({
            let name = stringify!($name);
            let ret = crate::llvm::apply::functions::std_llvm_type!($codegen, $ret);
            let args = &[$(crate::llvm::apply::functions::std_llvm_type!($codegen, $x).into()),+];
            $codegen.module.add_function(name, ret.fn_type(args, false), None);
        })+
    }};
}

pub(crate) use std_function;
pub(crate) use std_llvm_type;
