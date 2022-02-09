use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

use oat_ast::Expression;

use super::parse_expression;
use crate::types::parse_type;
use crate::ws;

fn parse_carray(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(
            tuple((tag("new"), multispace1)),
            separated_pair(
                parse_type,
                ws(tag("[]")),
                delimited(
                    char('{'),
                    ws(separated_list0(ws(char(',')), ws(parse_expression))),
                    char('}'),
                ),
            ),
        ),
        |(ty, els)| Expression::CArr(ty, els),
    )(input)
}

fn parse_new_array(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(
            tuple((tag("new"), multispace1)),
            pair(
                ws(parse_type),
                delimited(char('['), ws(parse_expression), char(']')),
            ),
        ),
        |(ty, length)| Expression::NewArr(ty, Box::new(length)),
    )(input)
}

pub fn parse_array(input: &str) -> IResult<&str, Expression> {
    alt((parse_carray, parse_new_array))(input)
}

#[cfg(test)]
mod array_tests {
    use super::*;
    use oat_ast::Type;
    #[test]
    fn carray() {
        assert_eq!(
            parse_array("new int[]{ 1, 2, 3 }"),
            Ok((
                "",
                Expression::CArr(Type::Int, vec![1i64.into(), 2i64.into(), 3i64.into()])
            ))
        );
    }

    #[test]
    fn new_array() {
        assert_eq!(
            parse_array("new int[3]"),
            Ok(("", Expression::NewArr(Type::Int, Box::new(3i64.into()))))
        );
    }
}
