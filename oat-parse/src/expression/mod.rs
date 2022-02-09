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

mod array;
pub use array::*;

mod call;
pub use call::*;

mod length;
pub use length::*;

use crate::helper::parse_int;
use crate::ws;

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    ws(alt((
        parse_bool,
        parse_null,
        map(parse_int, Expression::CInt),
        parse_length,
        // parse_call,
        map(parse_identifier, Expression::Id),
    )))(input)
}

#[cfg(test)]
mod expression_tests {
    use super::*;

    #[test]
    fn length() {
        assert_eq!(
            parse_expression("length(a)"),
            Ok((
                "",
                Expression::Length(Box::new(Expression::Id(String::from("a"))))
            ))
        );
    }
}
