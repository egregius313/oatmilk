#![allow(dead_code)]

use indexmap::IndexMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::{map, map_opt, opt, value},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use oat_ast::*;

mod helper;
use helper::ws;

mod expression;
use expression::*;
use types::{parse_return_type, parse_type};

mod types;

fn eq(input: &str) -> IResult<&str, &str> {
    ws(tag("="))(input)
}

fn semi(input: &str) -> IResult<&str, &str> {
    ws(tag(";"))(input)
}

pub fn parenthesized<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(char('('), inner, char(')'))
}

// fn ignore_whitespace(input: &str) -> IResult<&str, &str> {
//     take_while(is_whitespace)(input)
// }

// fn parse_int(input: &str) -> IResult<&str, Expression> {
// }

fn parse_block(input: &str) -> IResult<&str, Vec<Statement>> {
    delimited(ws(char('{')), many0(ws(parse_statement)), ws(char('}')))(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    let return_ = || ws(tag("return"));
    #[inline]
    fn simple_stmt<'a, T>(
        c: impl FnMut(&'a str) -> IResult<&'a str, T>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, T> {
        terminated(c, semi)
    }
    alt((
        map(
            terminated(separated_pair(parse_expression, eq, parse_expression), semi),
            |(target, value)| Statement::Assignment(target, value),
        ),
        map(
            terminated(
                preceded(
                    ws(tag("var")),
                    separated_pair(parse_identifier, eq, parse_expression),
                ),
                semi,
            ),
            |(id, init)| Statement::Declaration(id, init),
        ),
        map(
            tuple((
                tag("if"),
                ws(parenthesized(parse_expression)),
                parse_block,
                map(opt(preceded(ws(tag("else")), parse_block)), |else_| {
                    else_.unwrap_or_default()
                }),
            )),
            |(_, condition, then, else_)| Statement::If {
                condition,
                then,
                else_,
            },
        ),
        map_opt(simple_stmt(parse_expression), |e| match e {
            Expression::Call(fun, args) => Some(Statement::SCall(*fun, args)),
            _ => None,
        }),
        map(simple_stmt(preceded(return_(), parse_expression)), |e| {
            Statement::Return(Some(e))
        }),
        value(Statement::Return(None), simple_stmt(return_())),
    ))(input)
    // if let Ok((input, (target, value))) =
    // {
    //     return Ok((input, Statement::Assignment(target, value)));
    // } else if let Ok((input, (id, init))) = terminated(
    //     preceded(
    //         ws(tag("var")),
    //         separated_pair(parse_identifier, eq, parse_expression),
    //     ),
    //     semi,
    // ) {
    //     return Ok((input, Statement::Declaration(id, init)));
    // }
}

#[cfg(test)]
mod statement_tests {
    use super::*;
    use oat_ast::{Expression, Statement};
    use oat_symbol::create_session_if_not_set_then;

    macro_rules! assert_parses {
        ($src: expr, $body: expr) => {
            create_session_if_not_set_then(|_| {
                let cb = || $body;
                assert_eq!(parse_statement($src), Ok(("", cb())))
            })
        };
    }

    #[test]
    fn assignment() {
        assert_parses!("x = 0;", {
            let x: Expression = "x".into();
            Statement::Assignment(x, (0i64).into())
        })
    }

    #[test]
    fn if_() {
        assert_parses!("if (x == 0) { y = 1; } else { y = 2; }", {
            let x: Expression = "x".into();
            let y: Expression = "y".into();

            Statement::If {
                condition: Expression::Binary {
                    op: oat_ast::BinaryOp::Eq,
                    left: Box::new(x),
                    right: Box::new(0i64.into()),
                },
                then: vec![Statement::Assignment(y.clone(), 1i64.into())],
                else_: vec![Statement::Assignment(y.clone(), 2i64.into())],
            }
        })
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;
    use oat_symbol::create_session_if_not_set_then;

    #[inline]
    fn test_parse_block(src: &str, statements: impl FnOnce() -> Vec<Statement>) {
        create_session_if_not_set_then(|_| assert_eq!(parse_block(src), Ok(("", statements()))))
    }

    #[test]
    fn simple_block() {
        assert_eq!(parse_block("{}"), Ok(("", vec![])));
    }

    #[test]
    fn single_statment() {
        test_parse_block("{ x=0; }", || {
            let x: Expression = "x".into();
            vec![Statement::Assignment(x, 0i64.into())]
        })
    }

    #[test]
    fn multi_statement() {
        test_parse_block("{ x=0; x=1; }", || {
            let x: Expression = "x".into();
            vec![
                Statement::Assignment(x.clone(), 0i64.into()),
                Statement::Assignment(x.clone(), 1i64.into()),
            ]
        })
    }
}

// #[derive(Debug, PartialEq)]
// pub struct FunctionDecl {
//     pub return_type: ReturnType,
//     pub name: Id,
//     pub args: Vec<(Type, Id)>,
//     pub body: Block,
// }

fn parse_argspec(input: &str) -> IResult<&str, (Type, Id)> {
    let (input, type_) = parse_type(input)?;
    let (input, _space) = multispace0(input)?;
    let (input, name) = parse_identifier(input)?;

    Ok((input, (type_, name)))
}

fn parse_function_declaration(input: &str) -> IResult<&str, FunctionDecl> {
    let (input, return_type) = parse_return_type(input)?;
    let (input, name) = ws(parse_identifier)(input)?;
    let (input, args) = delimited(
        char('('),
        separated_list0(ws(char(',')), parse_argspec),
        char(')'),
    )(input)?;
    let (input, body) = ws(parse_block)(input)?;
    Ok((
        input,
        FunctionDecl {
            return_type,
            name,
            args,
            body,
        },
    ))
}

fn parse_type_declaration(input: &str) -> IResult<&str, TypeDeclaration> {
    let (input, _) = tag("struct")(input)?;
    let (input, name) = ws(parse_identifier)(input)?;
    let (input, field_decls) = delimited(
        char('{'),
        many0(ws(terminated(parse_argspec, tag(";")))),
        char('}'),
    )(input)?;

    let mut fields: IndexMap<Id, Type> = IndexMap::new();
    for (type_, name) in field_decls.into_iter() {
        fields.insert(name, type_);
    }

    Ok((input, TypeDeclaration { name, fields }))
}

fn parse_global_def(_input: &str) -> IResult<&str, GlobalDeclaration> {
    todo!("Global variable")
}

fn parse_declaration(input: &str) -> IResult<&str, Declaration> {
    alt((
        map(parse_function_declaration, Declaration::Function),
        map(parse_type_declaration, Declaration::Type),
        //map(parse_global_def, Declaration::Variable),
    ))(input)
}

#[cfg(test)]
mod declaration_tests {
    use super::*;
    use oat_symbol::create_session_if_not_set_then;

    #[inline]
    fn test_declaration(src: &str, declaration: impl FnOnce() -> Declaration) {
        create_session_if_not_set_then(|_| {
            assert_eq!(parse_declaration(src), Ok(("", declaration())))
        })
    }

    #[test]
    fn simple_function() {
        test_declaration("void f() {}", || {
            Declaration::Function(FunctionDecl {
                return_type: ReturnType::ReturnVoid,
                name: "f".into(),
                args: vec![],
                body: vec![],
            })
        })
    }

    #[test]
    fn one_arg() {
        test_declaration("void f(int x) {}", || {
            Declaration::Function(FunctionDecl {
                return_type: ReturnType::ReturnVoid,
                name: "f".into(),
                args: vec![(Type::Int, "x".into())],
                body: vec![],
            })
        })
    }

    #[test]
    fn empty_struct() {
        test_declaration("struct empty {}", || {
            Declaration::Type(TypeDeclaration {
                name: "empty".into(),
                fields: Default::default(),
            })
        })
    }

    #[test]
    fn point() {
        test_declaration("struct point { int x; int y; }", || {
            use indexmap::indexmap;

            let expected = TypeDeclaration {
                name: "point".into(),
                fields: indexmap! {
                    "x".into() => Type::Int,
                    "y".into() => Type::Int,
                },
            };

            Declaration::Type(expected)
        })
    }
}

fn parse_program_internal(input: &str) -> IResult<&str, Program> {
    //let mut declarations: Vec<Declaration> = vec![];

    // while let Ok((new_input, declaration)) = ws(parse_declaration)(input) {
    //     declarations.push(declaration);
    //     input = new_input;
    // }
    let (input, declarations) = many0(ws(parse_declaration))(input)?;
    Ok((input, Program { declarations }))
}

#[derive(Debug)]
pub struct OatParseError;

impl std::fmt::Display for OatParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oat parser error")
    }
}

impl std::error::Error for OatParseError {
    fn description(&self) -> &str {
        "The Oat Parser failed"
    }
}

// TODO: Replace with proper error type
pub fn parse_program(input: &str) -> Result<Program, OatParseError> {
    parse_program_internal(input)
        .and_then(|(_, p)| Ok(p))
        .or_else(|_| Err(OatParseError))

    // if let Ok((input, program)) = parse_program_internal(input) {
    //     Ok(program)
    // } else {
    //     Err(())
    // }
}

#[cfg(test)]
mod test_program {
    use super::*;
    use oat_symbol::create_session_if_not_set_then;

    #[test]
    fn point_program() -> Result<(), Box<dyn std::error::Error>> {
        // use oat_ast::Declaration::*;
        create_session_if_not_set_then(|_| {
            let src = concat!(
                // "int distance(point p1, point p2) { return (p1.x - p2.x)*(p1.x - p2.x) + (p1.y-p2.y)*(p1.y-p2.y); }\n",
                "struct empty {}\n",
                "void f() {  }\n",
                "struct point { int x; int y; }",
                "\n",
            );

            dbg!(parse_program(src))?;
            assert!(matches!(parse_program(src), Ok(Program { .. })));

            Ok(())
        })
    }
}
