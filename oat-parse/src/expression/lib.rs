use nom::{
    bytes::complete::tag,
    combinator::{alt, map},
    IResult,
};

use oat_ast::*;

fn parse_unop(input: &str) -> IResult<&str, UnaryOp> {
    alt((
        map(tag("-"), |_| UnaryOp::Neg),
        map(tag("!"), |_| UnaryOp::Lognot),
        map(tag("~"), |_| UnaryOp::Bitnot),
    ))(input)
}

fn parse_binop(input: &str) -> IResult<&str, BinaryOp> {
    alt((
        map(tag("+"), |_| BinaryOp::Add),
        map(tag("-"), |_| BinaryOp::Sub),
        map(tag("*"), |_| BinaryOp::Mul),
        map(tag("=="), |_| BinaryOp::Eq),
        map(tag("!="), |_| BinaryOp::Neq),
        map(tag("<"), |_| BinaryOp::Lt),
        map(tag("<="), |_| BinaryOp::Lte),
        map(tag(">"), |_| BinaryOp::Gt),
        map(tag(">="), |_| BinaryOp::Gte),
        map(tag("&"), |_| BinaryOp::And),
        map(tag("|"), |_| BinaryOp::Or),
        map(tag("[&]", |_| BinaryOp::IAnd)),
        map(tag("[|]"), |_| BinaryOp::IOr),
        map(tag("<<"), |_| BinaryOp::Shl),
        map(tag(">>"), |_| BinaryOp::Shr),
        map(tag(">>>"), |_| BinaryOp::Sar),
    ))(input)
}
