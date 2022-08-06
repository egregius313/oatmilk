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

    #[error("Undefined varibale {0}")]
    UndefinedVariable(String),

    #[error("Array length must be integer, not {0:?}")]
    ArrayLength(Type),

    #[error("{array:?} array elements cannot be {elt:?}")]
    IncompatibleArrayElement { array: Type, elt: Type },

    #[error("Can only call functions")]
    CanOnlyCallFunctions,

    #[error("Cannot use void as an expression")]
    VoidExpression,

    #[error("Cannot assign to a function")]
    CannotAssignFunction,

    #[error("Return value expected, none provided")]
    ReturnValueMissing,

    #[error("Void functions cannot return values")]
    ReturnValueProvidedInVoidFunction,

    #[error("Function call expected {expected} args, found {given}")]
    IncompatibleFunctionArgCounts { expected: usize, given: usize },

    #[error("Dead code after return")]
    DeadCodeAfterReturn,

    #[error("Function does not return, but return value of type {expected_ret_type} is required")]
    DidNotReturn {
        expected_ret_type: oat_ast::ReturnType,
    },
}
