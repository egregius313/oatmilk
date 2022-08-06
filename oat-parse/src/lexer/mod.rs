use crate::helper::{parse_int, ws};

use super::tokens::Token;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, recognize},
    multi::many0,
    sequence::pair,
    IResult,
};

macro_rules! simple_tokens {
    ($($tag: expr => $value: expr,)*) => {
        alt(($(::nom::combinator::value($value, ::nom::bytes::complete::tag($tag))),*))
    };
}

/// Use Rust style identifiers
fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )(input)
}

pub(crate) fn next_token(input: &str) -> IResult<&str, Token> {
    alt((
        simple_tokens! {
            "struct" => Token::Struct,
            "null" => Token::Null,
            "void" => Token::TVoid,
            "int" => Token::TInt,
            "string" => Token::TString,
            "else" => Token::Else,
            "if" => Token::If,
            "if?" => Token::Ifq,
            "for" => Token::For,
            "while" => Token::While,
            "var" => Token::Var,
            "return" => Token::Return,
        },
        simple_tokens! {
            "." => Token::Dot,
            "==" => Token::Eqeq,
            "=" => Token::Eq,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "[" => Token::LBracket,
            "]" => Token::RBracket,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "+" => Token::Plus,
            "-" => Token::Dash,
            "*" => Token::Star,
            "<<" => Token::Ltlt,
            "<=" => Token::Lteq,
            "<" => Token::Lt,
            ">=" => Token::Gteq,
            ">>" => Token::Gtgt,
            ">" => Token::Gt,
            ";" => Token::Semi,
        },
        map(parse_identifier, Token::Ident),
        map(parse_int, Token::Int),
    ))(input)
}

pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    many0(ws(next_token))(input)
}

#[cfg(test)]
mod tests {
    use crate::{lexer::tokenize, tokens::Token};

    #[test]
    fn lex_ex_1() {
        if let Ok((remaining, tokens)) = tokenize("var x = null;") {
            assert_eq!(
                tokens,
                vec![
                    Token::Var,
                    Token::Ident("x".to_string()),
                    Token::Eq,
                    Token::Null,
                    Token::Semi
                ]
            );
            assert_eq!(remaining, "");
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn lex_ex_2() {
        if let Ok((remaining, tokens)) =
            tokenize("for (var x = 1; x < 10; x = x + 1) { console.log(x); }")
        {
            let x = Token::Ident("x".to_string());
            assert_eq!(
                tokens,
                vec![
                    Token::For,
                    Token::LParen,
                    Token::Var,
                    x.clone(),
                    Token::Eq,
                    Token::Int(1),
                    Token::Semi,
                    x.clone(),
                    Token::Lt,
                    Token::Int(10),
                    Token::Semi,
                    x.clone(),
                    Token::Eq,
                    x.clone(),
                    Token::Plus,
                    Token::Int(1),
                    Token::RParen,
                    Token::LBrace,
                    Token::Ident("console".to_string()),
                    Token::Dot,
                    Token::Ident("log".to_string()),
                    Token::LParen,
                    x.clone(),
                    Token::RParen,
                    Token::Semi,
                    Token::RBrace,
                ]
            );
            assert_eq!(remaining, "");
        } else {
            assert_eq!(true, false);
        }
    }
}
