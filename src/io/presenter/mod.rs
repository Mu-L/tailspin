use miette::Result;

pub mod empty;
pub mod less;

pub trait Present: Send {
    fn present(&self) -> Result<()>;
}
