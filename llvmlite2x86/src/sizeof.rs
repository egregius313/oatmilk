//! Implementation of calculating the size of a type
//!
//! Necessary for implementing get element pointer

use llvmlite::{Type, TypeContext};

pub fn sizeof(tc: &TypeContext, ty: &Type) -> usize {
    use llvmlite::Type::*;
    match ty {
        Void => 0,
        I1 | I64 | Ptr(_) => 8,
        Struct(types) => types.iter().map(|t| sizeof(tc, t)).sum(),
        Array(length, type_) => length * sizeof(tc, type_),
        Namedt(id) => sizeof(tc, tc.get(id).unwrap()),
        _ => 0,
    }
}
