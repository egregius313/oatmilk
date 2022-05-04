use nom::multi::{fold_many0, separated_list0};
use nom::sequence::preceded;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt},
    sequence::{delimited, pair},
    IResult,
};
use oat_ast::Expression;

mod identifier;
pub use identifier::*;

mod boolean;
pub use boolean::*;

mod operator;
pub use operator::*;

mod null;
pub use null::*;

mod array;
pub use array::*;

mod call;
pub use call::*;

mod length;
pub use length::*;

mod string;
pub use string::*;

use crate::helper::parse_int;
use crate::ws;

#[derive(PartialEq, Clone, Debug)]
enum Suffix {
    Call(Vec<oat_ast::Expression>),
    Index(oat_ast::Expression),
    Projection(oat_ast::Id),
}

fn parse_suffix(input: &str) -> IResult<&str, Suffix> {
    ws(alt((
        map(
            delimited(
                char('('),
                separated_list0(char(','), parse_expression),
                char(')'),
            ),
            Suffix::Call,
        ),
        map(
            delimited(char('['), parse_expression, char(']')),
            Suffix::Index,
        ),
        map(preceded(char('.'), parse_identifier), |field| {
            Suffix::Projection(field)
        }),
    )))(input)
}

#[test]
fn test_suffix() {
    oat_symbol::create_session_if_not_set_then(|_| {
        assert_eq!(
            parse_suffix("(1)"),
            Ok(("", Suffix::Call(vec![Expression::CInt(1i64)])))
        );

        assert_eq!(
            parse_suffix("(1, x)"),
            Ok(("", Suffix::Call(vec![Expression::CInt(1i64), "x".into()])))
        );

        assert_eq!(parse_suffix("[x]"), Ok(("", Suffix::Index("x".into()))));

        assert_eq!(
            parse_suffix(".name"),
            Ok(("", Suffix::Projection("name".into())))
        )
    })
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, prefix) = opt(ws(parse_unop))(input)?;

    let (input, base) = ws(alt((
        parse_bool,
        parse_null,
        map(parse_int, Expression::CInt),
        parse_length,
        // parse_call,
        map(parse_identifier, Expression::Id),
        map(parse_string, Expression::CStr),
        delimited(char('('), ws(parse_expression), char(')')),
    )))(input)?;

    let (input, exp) = fold_many0(
        parse_suffix,
        move || base.clone(),
        |e, suffix| match suffix {
            Suffix::Call(args) => Expression::Call(Box::new(e), args),
            Suffix::Index(index) => Expression::Index {
                value: Box::new(e),
                index: Box::new(index),
            },
            Suffix::Projection(field) => Expression::Proj(Box::new(e), field),
        },
    )(input)?;

    let exp = match prefix {
        Some(op) => Expression::Unary(op, Box::new(exp)),
        None => exp,
    };

    let (input, next) = opt(pair(ws(parse_binop), parse_expression))(input)?;

    Ok((
        input,
        match next {
            Some((op, rhs)) => Expression::Binary {
                op,
                left: Box::new(exp),
                right: Box::new(rhs),
            },
            None => exp,
        },
    ))
}

#[cfg(test)]
mod expression_tests {
    use super::*;
    use oat_ast::Id;
    use oat_symbol::create_session_if_not_set_then;
    use Expression::*;

    macro_rules! assert_parses {
        ($text: expr, $expr: expr) => {
            create_session_if_not_set_then(|_| {
                let cb = || $expr;
                assert_eq!(parse_expression($text), Ok(("", cb())))
            })
        };
    }

    #[test]
    fn negative() {
        assert_parses!("-x", {
            let x: Expression = "x".into();
            Unary(oat_ast::UnaryOp::Neg, Box::new(x))
        })
    }

    #[test]
    fn string() {
        assert_parses!("\"hello\"", CStr("hello".to_string()))
    }

    #[test]
    fn length() {
        assert_parses!("length(a)", Length(Box::new("a".into())))
    }

    #[test]
    fn call_empty() {
        assert_parses!("f()", Call(Box::new("f".into()), vec![]))
    }

    #[test]
    fn call_several() {
        assert_parses!("g(x, y, 0, 1)", {
            let g: Expression = "g".into();
            let x: Expression = "x".into();
            let y: Expression = "y".into();
            Call(Box::new(g), vec![x, y, 0i64.into(), 1i64.into()])
        })
    }

    #[test]
    fn nested_calls() {
        assert_parses!("f(g())", {
            let f: Expression = "f".into();
            let g: Expression = "g".into();

            Call(Box::new(f), vec![Call(Box::new(g), vec![])])
        });
    }

    #[test]
    fn negated_compound() {
        assert_parses!("!f(x, y)", {
            let f: Expression = "f".into();
            let x: Expression = "x".into();
            let y: Expression = "y".into();
            Unary(
                oat_ast::UnaryOp::Lognot,
                Box::new(Call(Box::new(f), vec![x, y])),
            )
        })
    }

    #[test]
    fn binary_expression() {
        assert_parses!(
            "2 + 3",
            Binary {
                op: oat_ast::BinaryOp::Add,
                left: Box::new(CInt(2)),
                right: Box::new(CInt(3))
            }
        )
    }

    #[test]
    fn complex_call() {
        assert_parses!("request.headers[\"User-Agent\"].browser()", {
            let request: Expression = "request".into();
            let headers: Id = "headers".into();
            let user_agent: Expression = CStr("User-Agent".to_string());
            let browser: Id = "browser".into();

            let request_headers = Proj(Box::new(request), headers);
            let req_head_user_agent = Index {
                value: Box::new(request_headers),
                index: Box::new(user_agent),
            };
            let rh_ua_browser = Proj(Box::new(req_head_user_agent), browser);

            Call(Box::new(rh_ua_browser), vec![])
        })
    }
}
