#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, one_of},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use oat_ast::*;

mod helper;
use helper::{parse_int, ws};

mod expression;
use expression::*;

mod types;
use types::*;

fn eq(input: &str) -> IResult<&str, &str> {
    ws(tag("="))(input)
}

fn semi(input: &str) -> IResult<&str, &str> {
    ws(tag(";"))(input)
}

// fn ignore_whitespace(input: &str) -> IResult<&str, &str> {
//     take_while(is_whitespace)(input)
// }

// fn parse_int(input: &str) -> IResult<&str, Expression> {
// }

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

#[cfg(test)]
mod statement_tests {
    use super::*;
    #[test]
    fn assignment() {
        let x = Expression::Id(String::from("x"));

        assert_eq!(
            parse_statement("x = 0;"),
            Ok(("", Statement::Assignment(x, (0).into())))
        );
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;

    #[test]
    fn simple_block() {
        assert_eq!(parse_block(""), Ok(("", vec![])));
    }

    #[test]
    fn single_statment() {
        let x = Expression::Id(String::from("x"));
        assert_eq!(
            parse_block("x = 0;"),
            Ok(("", vec![Statement::Assignment(x, (0).into())]))
        )
    }

    #[test]
    fn multi_statement() {
        let x = Expression::Id(String::from("x"));

        assert_eq!(
            parse_block("x=0;\nx=1;"),
            Ok((
                "",
                vec![
                    Statement::Assignment(x.clone(), (0).into()),
                    Statement::Assignment(x.clone(), (1).into())
                ]
            ))
        )
    }
}

fn parse_declaration(_input: &str) -> IResult<&str, Declaration> {
    todo!("Declarations")
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, declarations) = many0(ws(parse_declaration))(input)?;
    Ok((input, Program { declarations }))
}
