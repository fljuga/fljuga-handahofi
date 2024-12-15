use winnow::ascii::*;
use winnow::combinator::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::error::*;
use winnow::stream::{AsChar, Stream};
use winnow::token::*;
use winnow::PResult;
use winnow::*;

use crate::grammar::tokens::internal;

#[derive(Debug)]
struct ClassDefinition<'a> {
    name: &'a str,
    properties: Vec<ClassProperty<'a>>,
}

#[derive(Debug)]
struct ClassProperty<'a> {
    name: &'a str,
    ty: &'a str,
}

// fn class_def<'a>(input: &mut &'a str) -> PResult<ClassDefinition<'a>> {
//     let name = preceded((literal("class"), space1), internal::identifier);
//     let bracket = (space1, literal('{'), space0);
//     let body = terminated(repeat(0.., class_property), (space0, literal('}')));
//
//     separated_pair(name, bracket, body).map(|(name, body)| ClassDefinition{ name, properties: vec![] })
//         .parse_next(input)
// }
//
// fn class_property<'a>(input: &mut &'a str) -> PResult<ClassProperty<'a>> {
//     "x".map(|_| ClassProperty{name: "x", ty: "x"}).parse_next(input)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use internal::tests::*;

    #[test]
    fn should_parse_class_def() {
        // test_parser(
        //     vec![
        //         ("class Name {", Some("Name"), ""), // Valid class, fully consumed
        //         ("", None, ""),                     // Empty input should fail
        //     ],
        //     class_def,
        // );
    }
}
