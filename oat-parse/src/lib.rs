#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{map, map_res, recognize, value},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use oat_ast::*;

mod helper;
use helper::{parse_int, ws};

fn eq(input: &str) -> IResult<&str, &str> {
    ws(tag("="))(input)
}

fn semi(input: &str) -> IResult<&str, &str> {
    ws(tag(";"))(input)
}

// fn ignore_whitespace(input: &str) -> IResult<&str, &str> {
//     take_while(is_whitespace)(input)
// }

/// Use Rust style identifiers
fn parse_identifier(input: &str) -> IResult<&str, Id> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |id: &str| String::from(id),
    )(input)
}

// fn parse_int(input: &str) -> IResult<&str, Expression> {
// }

fn parse_bool(input: &str) -> IResult<&str, Expression> {
    map(
        alt((value(true, tag("true")), value(false, tag("false")))),
        Expression::CBool,
    )(input)
    // let true_false = alt((tag("true"), tag("false")));
    // let (input, b) = map_res(true_false, |s: &str| s.parse::<bool>())(input)?;
    // Ok((input, Expression::CBool(b)))
}

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

fn parse_null(input: &str) -> IResult<&str, Expression> {
    let (input, reftype) = parse_reftype(input)?;
    let (input, _) = tag("NULL")(input)?;
    Ok((input, Expression::CNull(reftype)))
}

#[test]
fn null_tests() {
    assert_eq!(
        parse_null("string NULL"),
        Ok(("", Expression::CNull(ReferenceType::String)))
    );
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    ws(alt((
        parse_bool,
        parse_null,
        map(parse_int, Expression::CInt),
    )))(input)
}

#[test]
fn expression_tests() {
    use nom::error::{Error, ErrorKind};
    use nom::Err;
    assert_eq!(parse_expression("true"), Ok(("", Expression::CBool(true))));
    assert_eq!(
        parse_expression("false"),
        Ok(("", Expression::CBool(false)))
    );
    assert_eq!(
        parse_expression("True"),
        Err(Err::Error(Error::new("True", ErrorKind::Tag)))
    );
    assert_eq!(
        parse_expression("string NULL"),
        Ok(("", Expression::CNull(ReferenceType::String)))
    );
}

fn parse_block(input: &str) -> IResult<&str, Vec<Statement>> {
    many0(ws(parse_statement))(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        map(
            terminated(separated_pair(parse_expression, eq, parse_expression), semi),
            |(target, value)| Statement::Assignment(target, value),
        ),
        map(
            terminated(
                preceded(
                    ws(tag("var")),
                    separated_pair(parse_identifier, eq, parse_expression),
                ),
                semi,
            ),
            |(id, init)| Statement::Declaration(id, init),
        ),
    ))(input)
    // if let Ok((input, (target, value))) =
    // {
    //     return Ok((input, Statement::Assignment(target, value)));
    // } else if let Ok((input, (id, init))) = terminated(
    //     preceded(
    //         ws(tag("var")),
    //         separated_pair(parse_identifier, eq, parse_expression),
    //     ),
    //     semi,
    // ) {
    //     return Ok((input, Statement::Declaration(id, init)));
    // }
}

fn parse_declaration(input: &str) -> IResult<&str, Declaration> {
    todo!("Parse declaration")
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, declarations) = many0(ws(parse_declaration))(input)?;
    Ok((input, Program { declarations }))
}
