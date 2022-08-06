//! Definintion of a typing context, which associates identifiers with struct
//! definitions.
use std::iter::zip;

use indexmap::IndexMap;

use oat_ast as oat;
use oat_ast::{ReferenceType, ReturnType, Type};

use oat_error::TypeError;

pub type FieldSet = IndexMap<oat::Id, oat::Type>;

/// The typing context
#[derive(Default, Clone, Debug)]
pub struct TypingContext(IndexMap<oat::Id, FieldSet>);

impl TypingContext {
    /// Create a new typing context. Constructs an empty map.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get the type associated with `name`.
    pub fn get_type(&self, name: &oat::Id) -> Option<&FieldSet> {
        self.0.get(name)
    }

    /// Return the index and type of a field
    pub fn get_field(
        &self,
        type_name: &oat::Id,
        field_name: &oat::Id,
    ) -> Option<(usize, &oat::Type)> {
        let struct_ = self.0.get(type_name)?;
        let (i, _, type_) = struct_.get_full(field_name)?;
        Some((i, type_))
    }

    /// Create a type context from a collection of top-level type declarations.
    pub fn from_declarations(declarations: &Vec<oat::TypeDeclaration>) -> Self {
        let mut tc = Self::default();

        for oat::TypeDeclaration { name, fields } in declarations.iter() {
            tc.0.insert(*name, fields.clone());
        }

        tc
    }

    pub fn is_subtype(&self, sub: &Type, super_: &Type) -> Result<bool, TypeError> {
        use Type::*;
        if sub == super_ {
            return Ok(true);
        }
        Ok(match (sub, super_) {
            (NullRef(sub), NullRef(super_))
            | (Ref(sub), NullRef(super_))
            | (Ref(sub), Ref(super_)) => self.is_ref_subtype(sub, super_)?,
            _ => false,
        })
    }

    fn is_ref_subtype(
        &self,
        sub: &ReferenceType,
        super_: &ReferenceType,
    ) -> Result<bool, TypeError> {
        use oat_ast::ReferenceType::*;
        Ok(match (sub, super_) {
            (String, String) => true,
            (Array(t1), Array(t2)) => self.is_subtype(&*t1, &*t2)?,
            (Struct(s1), Struct(s2)) => {
                let sub_struct = self.get_type(&s1).ok_or(TypeError::StructNotFound(*s1))?;
                let super_struct = self.get_type(&s2).ok_or(TypeError::StructNotFound(*s2))?;

                if sub_struct.len() > super_struct.len() {
                    return Ok(false);
                }

                let field_sets = zip(sub_struct.iter(), super_struct.iter());

                for ((f1, t1), (f2, t2)) in field_sets {
                    if f1 != f2 || !self.is_subtype(t1, t2)? {
                        return Ok(false);
                    }
                }

                true
            }
            (Function(args1, ret1), Function(args2, ret2)) => {
                if args1.len() != args2.len() || !self.is_return_subtype(ret1, ret2)? {
                    return Ok(false);
                }
                let arg_pairs = zip(args1.iter(), args2.iter());
                for (a1, a2) in arg_pairs {
                    if !self.is_subtype(a2, a1)? {
                        return Ok(false);
                    }
                }
                true
            }
            _ => false,
        })
    }

    fn is_return_subtype(&self, sub: &ReturnType, super_: &ReturnType) -> Result<bool, TypeError> {
        use oat_ast::ReturnType::*;
        Ok(match (sub, super_) {
            (ReturnVoid, ReturnVoid) => true,
            (ReturnValue(r1), ReturnValue(r2)) => self.is_subtype(&r1, &r2)?,
            _ => false,
        })
    }
}
