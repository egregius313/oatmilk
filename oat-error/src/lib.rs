use thiserror::Error;

mod parser;
pub use parser::ParseError;

mod types;
pub use types::TypeError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Type error: {0}")]
    TypeError(#[from] TypeError),

    #[error("Parser Error: {0}")]
    ParserError(#[from] ParseError),
}
