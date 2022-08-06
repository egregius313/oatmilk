//! Implementation of type checking for the Oat language.
//!
//! Type checking is wrapped behind the [`type_check`] function.
//!
//! [`type_check`]: fn@type_check

use std::borrow::Borrow;
use std::collections::{HashSet, LinkedList};

use indexmap::IndexMap;

use oat::{Expression, FunctionDecl, GlobalDeclaration, ReferenceType, ReturnType, Statement};
use oat_ast as oat;
use oat_ast::{Id, Type};
use oat_typecontext::TypingContext;

use oat_error::TypeError;

mod locals_context;
use locals_context::LocalsContext;

/// Trait for making sure things can be type-checked.
///
/// Associated type `Output` is for whatever extra information needs to be
/// returned.
trait TypeCheck {
    type Output;
    fn type_check(
        &self,
        tc: &mut TypingContext,
        lc: &mut LocalsContext<Type>,
    ) -> Result<Self::Output, TypeError>;
}

fn tc_type(
    type_: &Type,
    tc: &TypingContext,
    lc: &mut LocalsContext<Type>,
    seen: &mut LinkedList<Id>,
) -> Result<(), TypeError> {
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
                        tc_type(field_type, tc, lc, seen)?;
                    }
                    Ok(())
                })?;
            seen.pop_front();
            Ok(())
        }
        Ref(Array(t)) | NullRef(Array(t)) => tc_type(t, tc, lc, seen),
        Ref(Function(args, ret)) | NullRef(Function(args, ret)) => {
            for arg_type in args.iter() {
                tc_type(arg_type, tc, lc, seen)?;
            }

            match ret.borrow() {
                oat::ReturnType::ReturnVoid => (),
                oat::ReturnType::ReturnValue(t) => {
                    tc_type(t, tc, lc, seen)?;
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

impl TypeCheck for oat::Type {
    type Output = ();
    fn type_check(
        &self,
        tc: &mut TypingContext,
        lc: &mut LocalsContext<Type>,
    ) -> Result<(), TypeError> {
        tc_type(self, tc, lc, &mut LinkedList::<oat::Id>::new())
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

    fn type_check(
        &self,
        tc: &mut TypingContext,
        lc: &mut LocalsContext<Type>,
    ) -> Result<Type, TypeError> {
        use oat_ast::Expression::*;
        Ok(match self {
            CNull(rt) => Type::NullRef(rt.clone()),
            CBool(_) => Type::Bool,
            CInt(_) => Type::Int,
            CStr(_) => Type::Ref(oat_ast::ReferenceType::String),
            Id(name) => lc
                .lookup(*name)
                .ok_or_else(|| TypeError::UndefinedVariable(name.name().to_string()))?,
            Length(e) => match e.type_check(tc, lc)? {
                Type::Ref(oat_ast::ReferenceType::Array(_)) => Type::Int,
                t => return Err(TypeError::CannotGetLength(t)),
            },
            Index { value, index } => match value.type_check(tc, lc)? {
                Type::Ref(oat::ReferenceType::Array(t)) => match index.type_check(tc, lc)? {
                    Type::Int => *t,
                    ti => return Err(TypeError::NonIntegerIndex(ti)),
                },
                t => return Err(TypeError::CannotSubscript(t)),
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
                    let type_ = expr.type_check(tc, lc)?;
                    let compatible = tc.is_subtype(&type_, field_type)?;
                    if !compatible {
                        return Err(TypeError::IncompatibleType);
                    }
                }
                Type::Ref(oat::ReferenceType::Struct(*struct_name))
            }
            Proj(e, field) => match e.type_check(tc, lc)? {
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
                _ => return Err(TypeError::IncompatibleType),
            },
            Unary(unop, nested) => {
                let nested_type = nested.type_check(tc, lc)?;
                let (expected_type, resulting_type) = unop.op_type();

                if nested_type != expected_type {
                    return Err(TypeError::IncompatibleType);
                }

                resulting_type
            }
            Binary { op, left, right } => {
                use oat_ast::BinaryOp::{Eq, Neq};
                match (
                    op.op_type(),
                    left.type_check(tc, lc)?,
                    right.type_check(tc, lc)?,
                ) {
                    (Some(((Type::Int, Type::Int), outt)), Type::Int, Type::Int) => outt,
                    (None, lt, rt) if lt == rt && matches!(op, Eq | Neq) => Type::Bool,
                    _ => return Err(TypeError::IncompatibleType),
                }
            }
            NewArr(type_, e) => match e.type_check(tc, lc)? {
                Type::Int => Type::Ref(ReferenceType::Array(Box::new(type_.clone()))),
                t => return Err(TypeError::ArrayLength(t)),
            },
            CArr(type_, elements) => {
                elements.iter().try_for_each(|e| {
                    let e_ty = e.type_check(tc, lc)?;
                    if !tc.is_subtype(&e_ty, type_)? {
                        return Err(TypeError::IncompatibleArrayElement {
                            array: type_.clone(),
                            elt: e_ty,
                        });
                    }
                    Ok(())
                })?;
                Type::Ref(ReferenceType::Array(Box::new(type_.clone())))
            }
            Call(fun, args) => {
                let (arg_types, ret_type) = match fun.type_check(tc, lc)? {
                    Type::Ref(ReferenceType::Function(arg_types, ret_type)) => {
                        (arg_types, ret_type)
                    }
                    _ => return Err(TypeError::CanOnlyCallFunctions),
                };
                std::iter::zip(arg_types.iter(), args.iter()).try_for_each(|(arg_type, arg)| {
                    let type_ = arg.type_check(tc, lc)?;
                    if !tc.is_subtype(&type_, arg_type)? {
                        return Err(TypeError::IncompatibleType);
                    }
                    Ok(())
                })?;
                match ret_type.borrow() {
                    ReturnType::ReturnVoid => return Err(TypeError::VoidExpression),
                    ReturnType::ReturnValue(t) => t.clone(),
                }
            }
        })
    }
}

fn type_check_arguments(
    arg_types: Vec<Type>,
    args: &Vec<Expression>,
    tc: &mut TypingContext,
    lc: &mut LocalsContext<Type>,
) -> Result<(), TypeError> {
    if arg_types.len() != args.len() {
        return Err(TypeError::IncompatibleFunctionArgCounts {
            expected: arg_types.len(),
            given: args.len(),
        });
    }

    std::iter::zip(arg_types.iter(), args.iter()).try_for_each(|(arg_type, arg)| {
        let passed_type = arg.type_check(tc, lc)?;
        if !tc.is_subtype(&passed_type, arg_type)? {
            return Err(TypeError::IncompatibleType);
        }
        Ok(())
    })?;
    Ok(())
}

#[must_use]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Returns(bool);

impl std::ops::BitAnd<Returns> for Returns {
    type Output = Returns;

    fn bitand(self, other: Returns) -> Self::Output {
        Returns(self.0 && other.0)
    }
}

impl std::ops::BitOr<Returns> for Returns {
    type Output = Returns;

    fn bitor(self, other: Returns) -> Self::Output {
        Returns(self.0 || other.0)
    }
}

fn type_check_statement(
    stmt: &Statement,
    tc: &mut TypingContext,
    lc: &mut LocalsContext<Type>,
    should_return: oat_ast::ReturnType,
) -> Result<Returns, TypeError> {
    use oat_ast::ReferenceType::Function;
    use oat_ast::Statement::*;
    use oat_ast::Type::Ref;

    let is_function = |name: &Id| matches!(lc.lookup(*name), Some(Ref(Function(..))));

    Ok(match stmt {
        Assignment(Expression::Id(name), _) if is_function(name) => {
            return Err(TypeError::CannotAssignFunction)
        }
        Declaration(name, e) => {
            let type_ = e.type_check(tc, lc)?;
            lc.set(*name, type_);
            Returns(false)
        }
        Return(None) if should_return == oat_ast::ReturnType::ReturnVoid => Returns(true),
        Return(None) => return Err(TypeError::ReturnValueMissing),
        Return(Some(rv)) => {
            let type_ = rv.type_check(tc, lc)?;
            match should_return {
                oat_ast::ReturnType::ReturnVoid => {
                    return Err(TypeError::ReturnValueProvidedInVoidFunction)
                }
                oat_ast::ReturnType::ReturnValue(ret_ty) => {
                    if tc.is_subtype(&type_, &ret_ty)? {
                        Returns(true)
                    } else {
                        return Err(TypeError::IncompatibleType);
                    }
                }
            }
        }
        SCall(fun, args) => match fun.type_check(tc, lc)? {
            Ref(Function(arg_types, ret_type)) if *ret_type == oat_ast::ReturnType::ReturnVoid => {
                type_check_arguments(arg_types, args, tc, lc)?;
                Returns(false)
            }
            _ => return Err(TypeError::IncompatibleType),
        },
        If {
            condition,
            then,
            else_,
        } => {
            match condition.type_check(tc, lc)? {
                Type::Bool => {}
                _ => return Err(TypeError::IncompatibleType),
            }
            let then_returns = type_check_block(then, tc, lc, should_return.clone())?;
            let else_returns = type_check_block(else_, tc, lc, should_return.clone())?;
            Returns(then_returns.0 && else_returns.0)
        }
        While { condition, body } => {
            match condition.type_check(tc, lc)? {
                Type::Bool => {}
                _ => return Err(TypeError::IncompatibleType),
            }
            type_check_block(body, tc, lc, should_return)?
        }

        _ => return Err(TypeError::IncompatibleType),
    })
}

fn type_check_block(
    block: &Vec<Statement>,
    tc: &mut TypingContext,
    lc: &LocalsContext<Type>,
    should_return: ReturnType,
) -> Result<Returns, TypeError> {
    let mut returns = false;
    let mut lc = lc.clone().new_child();
    for stmt in block {
        if returns {
            return Err(TypeError::DeadCodeAfterReturn);
        }
        match type_check_statement(stmt, tc, &mut lc, should_return.clone())? {
            Returns(true) => returns = true,
            _ => {}
        }
    }
    Ok(Returns(returns))
}

// impl TypeCheck for oat::Statement {
//     type Output = Returns;

//     fn type_check(
//         &self,
//         tc: &mut TypingContext,
//         lc: &mut LocalsContext<Type>,
//     ) -> Result<Self::Output, TypeError> {

//     }
// }

// let typecheck_fdecl (tc : Tctxt.t) (f : Ast.fdecl) (l : 'a Ast.node) : unit =
//   match f with
//   | { frtyp; fname; args; body } ->
//      let tc' = List.fold_left (fun tc (t, id) -> Tctxt.add_local tc id t) tc args in
//      let (_, returns) = typecheck_block tc' body frtyp in
//      if not returns
//      then type_error (List.hd body) ("Function " ^ fname ^ " has no return value")

impl TypeCheck for oat::FunctionDecl {
    type Output = ();

    fn type_check(
        &self,
        tc: &mut TypingContext,
        lc: &mut LocalsContext<Type>,
    ) -> Result<(), TypeError> {
        let FunctionDecl {
            return_type,
            args,
            body,
            name,
        } = self;

        let mut lc = {
            let mut lc = lc.clone().new_child();
            args.into_iter().for_each(|(t, a)| lc.set(*a, t.clone()));
            lc
        };

        let must_return = *return_type != ReturnType::ReturnVoid;
        let returns = type_check_block(body, tc, &mut lc, return_type.clone())?;

        // dbg!(must_return);
        // let _ = dbg!(returns);
        if must_return && !returns.0 {
            return Err(TypeError::DidNotReturn {
                expected_ret_type: return_type.clone(),
            });
        }

        println!("Succeeded type checking {}", name.name());

        Ok(())
    }
}

impl TypeCheck for oat::Program {
    type Output = ();

    fn type_check(
        &self,
        tc: &mut TypingContext,
        lc: &mut LocalsContext<Type>,
    ) -> Result<(), TypeError> {
        self.declarations
            .iter()
            .filter(|decl| matches!(decl, oat::Declaration::Function(..)))
            .try_for_each(|decl| {
                Ok(match decl {
                    oat::Declaration::Function(fdecl) => fdecl.type_check(tc, lc)?,
                    _ => {}
                })
            })?;
        Ok(())
    }
}

impl TypeCheck for oat::GlobalDeclaration {
    type Output = Type;

    fn type_check(
        &self,
        tc: &mut TypingContext,
        lc: &mut LocalsContext<Type>,
    ) -> Result<Type, TypeError> {
        use oat_ast::Expression::*;

        Ok(match &self.init {
            CNull(rt) => Type::NullRef(rt.clone()),
            CBool(_) => Type::Bool,
            CInt(_) => Type::Int,
            CStr(_) => Type::Ref(ReferenceType::String),
            Id(name) => match lc.lookup(name.clone()) {
                Some(t) => t,
                None => return Err(TypeError::UndefinedVariable(name.name().to_string())),
            },
            _ => todo!("TODO: Unimplemented global expression"),
        })
    }
}

/// Provides the type checking for a [`Program`].
///
/// # Arguments
///
/// * `prog` - the [`Program`] to check
///
/// # Return
///
/// Returns a [`Result`] with the error type [`TypeError`] that can be converted
/// to an [`oat_error::Error`].
///
/// [`Program`]: struct@oat_ast::Program
/// [`Result`]: enum@std::result::Result
/// [`TypeError`]: enum@oat_error::TypeError
/// [`oat_error::Error`]: enum@oat_error::Error
pub fn type_check(prog: &oat::Program) -> Result<(), TypeError> {
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
    let mut lc: LocalsContext<Type> = {
        let mut global = LocalsContext::<Type>::default();
        prog.clone().declarations.into_iter().try_for_each(|decl| {
            Ok(match decl {
                oat::Declaration::Function(oat::FunctionDecl {
                    return_type,
                    name,
                    args,
                    ..
                }) => {
                    let arg_types: Vec<Type> = args.into_iter().map(|(t, _)| t).collect();
                    global.set(
                        name,
                        oat::Type::Ref(oat::ReferenceType::Function(
                            arg_types,
                            Box::new(return_type),
                        )),
                    )
                }
                oat::Declaration::Variable(global_decl @ GlobalDeclaration { .. }) => {
                    let type_ = global_decl.type_check(&mut tc, &mut global)?;
                    global.set(global_decl.name, type_);
                }
                _ => {}
            })
        })?;
        global
    };
    // dbg!(tc.clone());
    // dbg!(lc.clone());
    prog.type_check(&mut tc, &mut lc)
}
