use thiserror::Error;

mod types;
pub use types::TypeError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Type error: {0}")]
    TypeError(#[from] TypeError),
}
