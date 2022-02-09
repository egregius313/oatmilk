use nom::{
    //    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use oat_ast::Expression;

use super::parse_expression;
use crate::ws;

pub fn parse_call(input: &str) -> IResult<&str, Expression> {
    map(
        tuple((
            parse_expression,
            delimited(
                char('('),
                separated_list0(char(','), ws(parse_expression)),
                char(')'),
            ),
        )),
        |(fun, args)| Expression::Call(Box::new(fun), args),
    )(input)
}
