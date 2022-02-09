use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};

use oat_ast::*;

pub fn parse_unop(input: &str) -> IResult<&str, UnaryOp> {
    alt((
        map(tag("-"), |_| UnaryOp::Neg),
        map(tag("!"), |_| UnaryOp::Lognot),
        map(tag("~"), |_| UnaryOp::Bitnot),
    ))(input)
}

pub fn parse_binop(input: &str) -> IResult<&str, BinaryOp> {
    alt((
        value(BinaryOp::Add, tag("+")),
        value(BinaryOp::Sub, tag("-")),
        value(BinaryOp::Mul, tag("*")),
        value(BinaryOp::Eq, tag("==")),
        value(BinaryOp::Neq, tag("!=")),
        value(BinaryOp::Lt, tag("<")),
        value(BinaryOp::Lte, tag("<=")),
        value(BinaryOp::Gt, tag(">")),
        value(BinaryOp::Gte, tag(">=")),
        value(BinaryOp::And, tag("&")),
        value(BinaryOp::Or, tag("|")),
        value(BinaryOp::IAnd, tag("[&]")),
        value(BinaryOp::IOr, tag("[|]")),
        value(BinaryOp::Shl, tag("<<")),
        value(BinaryOp::Shr, tag(">>")),
        value(BinaryOp::Sar, tag(">>>")),
    ))(input)
}
