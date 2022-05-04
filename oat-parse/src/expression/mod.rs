use nom::multi::{fold_many0, separated_list0};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    sequence::{delimited, tuple},
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

        assert_eq!(parse_suffix("[x]"), Ok(("", Suffix::Index("x".into()))))
    })
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, base) = ws(alt((
        parse_bool,
        parse_null,
        map(parse_int, Expression::CInt),
        parse_length,
        // parse_call,
        map(parse_identifier, Expression::Id),
        delimited(char('('), ws(parse_expression), char(')')),
    )))(input)?;

    fold_many0(
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
    )(input)
}

#[cfg(test)]
mod expression_tests {
    use super::*;
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
}
