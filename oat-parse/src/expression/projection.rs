use nom::{character::complete::char, sequence::tuple, IResult};

use oat_ast::Expression;

use super::{parse_expression, parse_identifier};

pub fn parse_projection(input: &str) -> IResult<&str, Expression> {
    let (input, (object, _, field)) =
        tuple((parse_expression, char('.'), parse_identifier))(input)?;

    Ok((input, Expression::Proj(Box::new(object), field)))
}

// #[cfg(test)]
// mod projection_tests {
//     use crate::expression::parse_projection;
//     use oat_ast::Expression;

//     #[test]
//     fn simple_dot_lookup() {
//         assert_eq!(
//             parse_projection("p.x"),
//             Ok((
//                 "",
//                 Expression::Proj(Box::new(Expression::Id("p".to_string())), "x".to_string())
//             ))
//         )
//     }

//     #[test]
//     fn call_dot_lookup() {
//         use Expression::*;

//         let x = "x".to_string();
//         let get_point = Id("get_point".to_string());

//         assert_eq!(
//             parse_projection("get_point(0, 0).x"),
//             Ok((
//                 "",
//                 Proj(
//                     Box::new(Call(Box::new(get_point), vec![0i64.into(), 0i64.into(),])),
//                     x
//                 )
//             ))
//         )
//     }
// }
