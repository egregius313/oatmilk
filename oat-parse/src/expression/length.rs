use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    sequence::{delimited, preceded},
    IResult,
};

use oat_ast::Expression;

use super::parse_expression;

pub fn parse_length(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(
            tag("length"),
            delimited(char('('), parse_expression, char(')')),
        ),
        |e| Expression::Length(Box::new(e)),
    )(input)
}
