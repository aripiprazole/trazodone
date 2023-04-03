pub mod spec_ir;
pub mod spec_syntax;
pub mod spec_imp;

pub type Result<T> = std::result::Result<T, String>;

pub trait Transform {
    type Output;

    fn transform(self) -> Result<Self::Output>;
}