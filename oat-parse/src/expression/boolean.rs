use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};
use oat_ast::Expression;

pub fn parse_bool(input: &str) -> IResult<&str, Expression> {
    map(
        alt((value(true, tag("true")), value(false, tag("false")))),
        Expression::CBool,
    )(input)
    // let true_false = alt((tag("true"), tag("false")));
    // let (input, b) = map_res(true_false, |s: &str| s.parse::<bool>())(input)?;
    // Ok((input, Expression::CBool(b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_tests() {
        use nom::error::{Error, ErrorKind};
        use nom::Err;
        assert_eq!(parse_bool("true"), Ok(("", Expression::CBool(true))));
        assert_eq!(parse_bool("false"), Ok(("", Expression::CBool(false))));
        assert_eq!(
            parse_bool("True"),
            Err(Err::Error(Error::new("True", ErrorKind::Tag)))
        );
    }
}
