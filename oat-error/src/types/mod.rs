use oat_ast::{Id, Type};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Struct not found: {0:?}")]
    StructNotFound(Id),

    #[error("Can only subscript arrays, not {0:?}")]
    CannotSubscript(Type),

    #[error("Index must be an integer, not {0:?}")]
    NonIntegerIndex(Type),

    #[error("Can only get the length of arrays, not {0:?}")]
    CannotGetLength(Type),

    #[error("Field {1:?} not found for type {0:?}")]
    FieldNotFound(Type, Id),

    #[error("Duplicate field {0:?}")]
    DuplicateField(Id),

    #[error("Missing required field {1:?} for type {0:?}")]
    MissingField(Type, Id),

    #[error("Incompatible types")]
    IncompatibleType,
}
