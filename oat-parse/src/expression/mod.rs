use nom::{branch::alt, combinator::map, IResult};
use oat_ast::Expression;

mod identifier;
pub use identifier::*;

mod boolean;
pub use boolean::*;

mod operator;
pub use operator::*;

mod null;
pub use null::*;

use crate::helper::parse_int;
use crate::ws;

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    ws(alt((
        parse_bool,
        parse_null,
        map(parse_int, Expression::CInt),
        map(parse_identifier, Expression::Id),
    )))(input)
}
