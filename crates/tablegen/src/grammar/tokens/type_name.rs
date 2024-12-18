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

//!
//! Tablegen token type name parsing.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

use crate::grammar::tokens::digits::*;
use crate::grammar::tokens::identifier::*;

pub(crate) fn generic_bits_type_name(input: &mut &str) -> PResult<i64> {
    delimited(literal("bits<"), int, literal(">")).parse_next(input)
}

pub(crate) fn generic_type_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    delimited(literal("type<"), identifier, literal(">")).parse_next(input)
}

fn type_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        "bit",
        "int",
        "string",
        "dag",
        // generic_bits_type_name,
        // generic_type_name,
        identifier,
    ))
        .parse_next(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_parse_generic_types() {
        test_parser(
            vec![
                ("bits<12>", Some(12), ""), // Valid prefixed identifier, fully consumed
                ("bits<15>xx", Some(15), "xx"), // Partially valid identifier input, stops before 'x'
                ("", None, ""),                 // Empty input should fail
            ],
            generic_bits_type_name,
        );

        test_parser(
            vec![
                ("type<typename>", Some("typename"), ""), // Valid prefixed identifier, fully consumed
                ("type<typename>xx", Some("typename"), "xx"), // Partially valid identifier input, stops before 'x'
                ("", None, ""),                               // Empty input should fail
            ],
            generic_type_name,
        );
    }
}