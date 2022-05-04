use std::borrow::Borrow;
use std::collections::{HashSet, LinkedList};

use indexmap::IndexMap;

use oat_ast as oat;
use oat_ast::{Id, Type};
use oat_typecontext::TypingContext;

use oat_error::{Error as OatError, TypeError};

trait TypeCheck {
    type Output;
    fn type_check(&self, tc: &mut TypingContext) -> Result<Self::Output, OatError>;
}

fn tc_type(type_: &Type, tc: &TypingContext, seen: &mut LinkedList<Id>) -> Result<(), TypeError> {
    use oat_ast::ReferenceType::{Array, Function, Struct};
    use oat_ast::Type::{NullRef, Ref};
    match type_ {
        Ref(Struct(name)) | NullRef(Struct(name)) if seen.contains(name) => Ok(()),
        Ref(Struct(name)) | NullRef(Struct(name)) => {
            seen.push_front(*name);
            tc.get_type(name)
                .ok_or(TypeError::StructNotFound(*name))
                .and_then(|fields| {
                    for field_type in fields.values() {
                        tc_type(field_type, tc, seen)?;
                    }
                    Ok(())
                })?;
            seen.pop_front();
            Ok(())
        }
        Ref(Array(t)) | NullRef(Array(t)) => tc_type(t, tc, seen),
        Ref(Function(args, ret)) | NullRef(Function(args, ret)) => {
            for arg_type in args.iter() {
                tc_type(arg_type, tc, seen)?;
            }

            match ret.borrow() {
                oat::ReturnType::ReturnVoid => (),
                oat::ReturnType::ReturnValue(t) => {
                    tc_type(&t, tc, seen)?;
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

impl TypeCheck for oat::Type {
    type Output = ();
    fn type_check(&self, tc: &mut TypingContext) -> Result<(), OatError> {
        Ok(tc_type(self, tc, &mut LinkedList::<oat::Id>::new())?)
    }
}

fn check_duplicate_fields(
    fields: Vec<(oat::Id, oat::Expression)>,
) -> Result<IndexMap<oat::Id, oat::Expression>, TypeError> {
    let mut field_names: HashSet<oat::Id> = HashSet::new();
    for (field_name, _) in fields.iter() {
        if field_names.contains(field_name) {
            return Err(TypeError::DuplicateField(*field_name));
        }
        field_names.insert(*field_name);
    }
    Ok(IndexMap::from_iter(fields))
}

impl TypeCheck for oat::Expression {
    type Output = Type;

    fn type_check(&self, tc: &mut TypingContext) -> Result<Type, OatError> {
        use oat_ast::Expression::*;
        Ok(match self {
            CNull(rt) => Type::NullRef(rt.clone()),
            CBool(_) => Type::Bool,
            CInt(_) => Type::Int,
            CStr(_) => Type::Ref(oat_ast::ReferenceType::String),
            Index { value, index } => match value.type_check(tc)? {
                Type::Ref(oat::ReferenceType::Array(t)) => match index.type_check(tc)? {
                    Type::Int => *t,
                    ti => Err(TypeError::NonIntegerIndex(ti))?,
                },
                t => Err(TypeError::CannotSubscript(t))?,
            },
            CStruct(struct_name, fields) => {
                let instance = check_duplicate_fields(fields.to_vec())?;
                let tc_ = tc.clone();
                let struct_def = tc_
                    .get_type(struct_name)
                    .ok_or(TypeError::StructNotFound(*struct_name))?;

                for (field_name, field_type) in struct_def.iter() {
                    let expected_type = Type::Ref(oat::ReferenceType::Struct(*struct_name));
                    let expr = instance
                        .get(field_name)
                        .ok_or(TypeError::MissingField(expected_type, *field_name))?;
                    let type_ = expr.type_check(tc)?;
                    let compatible = tc.is_subtype(&type_, field_type)?;
                    if !compatible {
                        Err(TypeError::IncompatibleType)?;
                    }
                }
                Type::Ref(oat::ReferenceType::Struct(*struct_name))
            }
            Proj(e, field) => match e.type_check(tc)? {
                Type::Ref(oat::ReferenceType::Struct(struct_name)) => {
                    let struct_def = tc
                        .get_type(&struct_name)
                        .ok_or(TypeError::StructNotFound(struct_name))?;
                    let struct_type = Type::Ref(oat::ReferenceType::Struct(struct_name));
                    struct_def
                        .get(field)
                        .ok_or(TypeError::FieldNotFound(struct_type, *field))?
                        .clone()
                }
                _ => Err(TypeError::IncompatibleType)?,
            },
            _ => todo!("{:?}", self),
        })
    }
}

impl TypeCheck for oat::Program {
    type Output = ();

    fn type_check(&self, tc: &mut TypingContext) -> Result<(), OatError> {
        Ok(())
    }
}

pub fn type_check(prog: &oat::Program) -> Result<(), OatError> {
    let type_declarations: Vec<oat::TypeDeclaration> = prog
        .clone()
        .declarations
        .into_iter()
        .filter_map(|decl| match decl {
            oat::Declaration::Type(td) => Some(td),
            _ => None,
        })
        .collect();
    let mut tc: TypingContext = TypingContext::from_declarations(&type_declarations);
    prog.type_check(&mut tc)
}
