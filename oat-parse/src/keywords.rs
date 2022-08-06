use nom::{bytes::complete::tag, IResult};

use crate::helper::ws;

macro_rules! keyword {
    ($name: ident, $tag: expr) => {
        #[inline]
        pub fn $name(input: &str) -> IResult<&str, ()> {
            let (input, _) = ws(tag($tag))(input)?;
            Ok((input, ()))
        }
    };
}

keyword!(var, "var");
keyword!(for_, "for");
keyword!(while_, "while");
keyword!(if_, "if");
keyword!(else_, "else");
keyword!(return_, "return");
