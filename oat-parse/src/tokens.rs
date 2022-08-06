use nom::{Compare, CompareResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Struct,
    Null,
    TVoid,
    TInt,
    TString,
    Else,
    If,
    Ifq,
    While,
    Return,
    Var,
    Global,
    Length,
    Dot,
    Semi,
    Comma,
    LBrace,
    RBrace,
    Plus,
    Dash,
    Star,
    Eq,
    Eqeq,
    Bang,
    Tilde,
    LParen,
    RParen,
    LBracket,
    RBracket,
    For,
    New,
    True,
    False,
    TBool,
    Ltlt,
    Gtgt,
    Gtgtgt,
    BangEq,
    Lt,
    Lteq,
    Gt,
    Gteq,
    Ampersand,
    Bar,
    IAnd,
    IOr,
    Arrow,
    Question,
    String(String),
    Ident(String),
    Int(i64),
}

#[derive(Debug, PartialEq, From, Into, AsRef)]
struct TokenStream<'a> {
    stream: &'a [Token],
}

impl<'a> TokenStream<'a> {
    fn len(&self) -> usize {
        self.stream.len()
    }
}

impl<'a, 'b> Compare<TokenStream<'b>> for TokenStream<'a> {
    #[inline(always)]
    fn compare(&self, t: TokenStream<'b>) -> CompareResult {
        let pos = self
            .stream
            .iter()
            .zip(t.stream.iter())
            .position(|(a, b)| a != b);

        match pos {
            Some(_) => CompareResult::Error,
            None => {
                if self.len() >= t.len() {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }

    #[inline]
    fn compare_no_case(&self, t: TokenStream<'b>) -> CompareResult {
        self.compare(t)
    }
}
