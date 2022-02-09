#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, recognize},
    multi::many0,
    sequence::pair,
    IResult,
};

use oat_ast::Id;

/// Use Rust style identifiers
pub fn parse_identifier(input: &str) -> IResult<&str, Id> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |id: &str| String::from(id),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_alpha() {
        assert_eq!(
            parse_identifier("variable").unwrap(),
            ("", String::from("variable"))
        );
    }
}
