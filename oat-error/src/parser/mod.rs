use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Remaining Input: {0:?}")]
    RemainingInput(String),

    #[error("Nom Parser Error")]
    NomParserError,
}
