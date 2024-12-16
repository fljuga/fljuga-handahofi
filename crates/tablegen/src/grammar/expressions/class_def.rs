/*
   Copyright (C) 2022-2024 Yuriy Yarosh.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::error::*;
use winnow::stream::{AsChar, Stream};
use winnow::token::*;
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
