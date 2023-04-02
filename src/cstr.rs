macro_rules! cstr {
    ($s:ident) => {
        format!("{}\0", $s).as_ptr() as *const i8
    };
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
    () => {
        "\0".as_ptr() as *const i8
    };
}

pub(crate) use cstr;
