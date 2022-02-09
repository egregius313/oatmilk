use crate::types::parse_reftype;
use nom::{
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::map,
    sequence::{terminated, tuple},
    IResult,
};
use oat_ast::Expression;

pub fn parse_null(input: &str) -> IResult<&str, Expression> {
    let null = tag("NULL");
    map(
        terminated(parse_reftype, tuple((multispace1, null))),
        Expression::CNull,
    )(input)
}

#[cfg(test)]
mod null_tests {
    use super::*;
    use oat_ast::{Expression, ReferenceType, Type};
    #[test]
    fn string() {
        assert_eq!(
            parse_null("string NULL"),
            Ok(("", Expression::CNull(ReferenceType::String)))
        );
    }

    #[test]
    fn array() {
        assert_eq!(
            parse_null("int[] NULL"),
            Ok((
                "",
                Expression::CNull(ReferenceType::Array(Box::new(Type::Int)))
            ))
        );
    }
}
