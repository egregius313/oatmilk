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
use oat_symbol::Symbol;

/// Use Rust style identifiers
pub fn parse_identifier(input: &str) -> IResult<&str, Id> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        Symbol::intern,
    )(input)
}

#[cfg(test)]
mod tests {
    use oat_symbol::create_session_if_not_set_then;

    use super::*;

    #[test]
    fn all_alpha() {
        create_session_if_not_set_then(|_| {
            assert_eq!(
                parse_identifier("variable").unwrap(),
                ("", Id::from("variable"))
            );
        })
    }
}
