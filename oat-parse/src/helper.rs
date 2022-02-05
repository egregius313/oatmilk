use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, one_of},
    combinator::{map_res, recognize},
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
    IResult,
};

fn hexadecimal(input: &str) -> IResult<&str, i64> {
    // <'a, E: ParseError<&'a str>>
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 16),
    )(input)
}

fn octal(input: &str) -> IResult<&str, i64> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 8),
    )(input)
}

fn binary(input: &str) -> IResult<&str, i64> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 2),
    )(input)
}

fn decimal(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 10),
    )(input)
}

pub fn parse_int(input: &str) -> IResult<&str, i64> {
    alt((hexadecimal, octal, binary, decimal))(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes both leading and trailing whitespace, returning the output of
/// `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
