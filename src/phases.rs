pub mod eval;
pub mod spec;

pub type Result<T> = std::result::Result<T, String>;

pub trait Transform {
    type Output;

    fn transform(self) -> Result<Self::Output>;
}
