use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, map_res, value},
    multi::{fold_many0, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};

use oat_ast::{ReferenceType, ReturnType, Type};

use crate::expression::parse_identifier;
use crate::ws;

pub fn parse_reftype(input: &str) -> IResult<&str, ReferenceType> {
    map_res(parse_type, |t| match t {
        Type::Ref(rt) => Ok(rt),
        _ => Err(()),
    })(input)
}

#[cfg(test)]
mod reference_type_tests {
    use super::*;
    #[test]
    fn string() {
        assert_eq!(parse_reftype("string"), Ok(("", ReferenceType::String)));
    }

    #[test]
    fn int_arr() {
        assert_eq!(
            parse_type("int[]"),
            Ok(("", Type::Ref(ReferenceType::Array(Box::new(Type::Int)))))
        );
    }

    // #[test]
    // fn my_class_arr() {
    //     use ReferenceType::{Array, Struct};
    //     let my_class = Type::Ref(Struct(String::from("MyClass")));
    //     assert_eq!(
    //         parse_reftype("MyClass[]"),
    //         Ok(("", Array(Box::new(my_class))))
    //     );
    // }
    // #[test]
    // fn fn_no_arg_ret_bool() {
    //     assert_eq!(
    //         parse_reftype("() -> bool"),
    //         Ok((
    //             "",
    //             ReferenceType::Function(vec![], Box::new(ReturnType::ReturnVoid))
    //         ))
    //     );
    // }

    #[test]
    fn type_list() {
        assert_eq!(
            delimited(
                char('('),
                ws(separated_list0(ws(char(',')), parse_type)),
                char(')'),
            )("()"),
            Ok(("", vec![]))
        );
    }
    #[test]
    fn fn_raw() {
        assert_eq!(
            map(
                separated_pair(
                    delimited(
                        char('('),
                        separated_list0(char(','), ws(parse_type)),
                        char(')'),
                    ),
                    ws(tag("->")),
                    parse_return_type,
                ),
                |(arg_types, ret_type)| ReferenceType::Function(arg_types, Box::new(ret_type)),
            )("() -> bool"),
            Ok((
                "",
                ReferenceType::Function(vec![], Box::new(ReturnType::ReturnValue(Type::Bool)))
            ))
        );
    }
}

// #[test]
// fn reference_type_tests() {
//     assert_eq!(
//         map(
//             separated_pair(
//                 delimited(
//                     char('('),
//                     separated_list0(char(','), ws(parse_type)),
//                     char(')'),
//                 ),
//                 ws(tag("->")),
//                 parse_return_type,
//             ),
//             |(arg_types, ret_type)| ReferenceType::Function(arg_types, Box::new(ret_type)),
//         )("() -> bool"),
//         Ok((
//             "",
//             ReferenceType::Function(vec![], Box::new(ReturnType::ReturnVoid))
//         ))
//     )
// }

#[derive(PartialEq, Clone, Copy)]
enum TypeSuffix {
    Null,
    Array,
}

fn parse_type_suffix(input: &str) -> IResult<&str, TypeSuffix> {
    alt((
        value(TypeSuffix::Null, tag("?")),
        value(TypeSuffix::Array, tag("[]")),
    ))(input)
}

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    let (input, init) = alt((
        map(tag("bool"), |_: &str| Type::Bool),
        map(tag("int"), |_: &str| Type::Int),
        map(tag("string"), |_| Type::Ref(ReferenceType::String)),
        map(parse_identifier, |id| Type::Ref(ReferenceType::Struct(id))),
        map(
            separated_pair(
                delimited(
                    char('('),
                    separated_list0(char(','), ws(parse_type)),
                    char(')'),
                ),
                ws(tag("->")),
                parse_return_type,
            ),
            |(arg_types, ret_type)| {
                Type::Ref(ReferenceType::Function(arg_types, Box::new(ret_type)))
            },
        ),
        delimited(char('('), parse_type, char(')')),
    ))(input)?;
    fold_many0(
        parse_type_suffix,
        move || init.clone(),
        |t, suffix| match (t.clone(), suffix) {
            (Type::Ref(rt), TypeSuffix::Null) => oat_ast::Type::NullRef(rt),
            (_, TypeSuffix::Array) => oat_ast::Type::Ref(ReferenceType::Array(Box::new(t))),
            (_, _) => t,
        },
    )(input)
}

#[cfg(test)]
mod type_tests {
    use super::*;
    use oat_ast::Id;
    use oat_symbol::create_session_if_not_set_then;
    #[test]
    fn string_arr() {
        assert_eq!(
            parse_type("string[]"),
            Ok((
                "",
                Type::Ref(ReferenceType::Array(Box::new(Type::Ref(
                    ReferenceType::String
                ))))
            ))
        );
    }

    #[test]
    fn paren_int() {
        assert_eq!(parse_type("(int)"), Ok(("", Type::Int)));
    }

    #[test]
    fn paren_int_arr() {
        assert_eq!(
            parse_type("(int[])"),
            Ok(("", Type::Ref(ReferenceType::Array(Box::new(Type::Int)))))
        );
    }
    #[test]
    fn nullable_string() {
        use ReferenceType::String;
        use Type::NullRef;
        assert_eq!(parse_type("string?"), Ok(("", NullRef(String))));
    }
    #[test]
    fn boolean() {
        assert_eq!(parse_type("bool"), Ok(("", Type::Bool)));
    }
    #[test]
    fn int() {
        assert_eq!(parse_type("int"), Ok(("", Type::Int)));
    }

    #[test]
    fn fn_ret_bool() {
        assert_eq!(
            parse_type("() -> bool"),
            Ok((
                "",
                Type::Ref(ReferenceType::Function(
                    vec![],
                    Box::new(ReturnType::ReturnValue(Type::Bool))
                ))
            ))
        );
    }
    #[test]
    fn my_class() {
        create_session_if_not_set_then(|_| {
            assert_eq!(
                parse_type("MyClass"),
                Ok(("", Type::Ref(ReferenceType::Struct(Id::from("MyClass")))))
            );
        })
    }
}

pub fn parse_return_type(input: &str) -> IResult<&str, ReturnType> {
    alt((
        map(tag("void"), |_| ReturnType::ReturnVoid),
        map(parse_type, ReturnType::ReturnValue),
    ))(input)
}

#[cfg(test)]
mod return_type_tests {
    use oat_ast::Id;
    use oat_symbol::create_session_if_not_set_then;

    use super::*;
    #[test]
    fn ret_void() {
        assert_eq!(parse_return_type("void"), Ok(("", ReturnType::ReturnVoid)));
    }

    #[test]
    fn ret_bool() {
        assert_eq!(
            parse_return_type("bool"),
            Ok(("", ReturnType::ReturnValue(Type::Bool)))
        );
    }

    #[test]
    fn ret_null_ref() {
        create_session_if_not_set_then(|_| {
            let my_class = ReferenceType::Struct(Id::from("MyClass"));
            assert_eq!(
                parse_return_type("MyClass?"),
                Ok(("", ReturnType::ReturnValue(Type::NullRef(my_class))))
            );
        })
    }

    #[test]
    fn ret_struct_ref() {
        create_session_if_not_set_then(|_| {
            let my_class = ReferenceType::Struct(Id::from("MyClass"));
            assert_eq!(
                parse_return_type("MyClass"),
                Ok(("", ReturnType::ReturnValue(Type::Ref(my_class))))
            );
        })
    }
}
