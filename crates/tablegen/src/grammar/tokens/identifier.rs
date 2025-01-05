/*
   Copyright (C) 2022-2025 Yuriy Yarosh.

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
//! Tablegen identifiers parsing.
//!

use winnow::error::*;
use winnow::token::*;
use winnow::PResult;
use winnow::*;

use crate::grammar::tokens::helpers::*;

const RESERVED_WORDS: [&str; 25] = [
    "assert",
    "bit",
    "bits",
    "class",
    "code",
    "dag",
    "def",
    "dump",
    "else",
    "false",
    "foreach",
    "defm",
    "defset",
    "defvar",
    "field",
    "if",
    "in",
    "include",
    "int",
    "let",
    "list",
    "multiclass",
    "string",
    "then",
    "true",
];

/// Parses identifier alpha+ chars including underscore.
pub(crate) fn alpha_identifier_chars1<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '_')).parse_next(input)
}

/// Parses identifier digit* chars.
pub(crate) fn digit_identifier_chars0<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(0.., '0'..='9').parse_next(input)
}

pub(crate) fn identifier<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let id = concat([
        digit_identifier_chars0 as StrParser<'a>,
        alpha_identifier_chars1 as StrParser<'a>,
        digit_identifier_chars0 as StrParser<'a>,
    ])
    .parse_next(input)?;

    if RESERVED_WORDS.contains(&id) {
        Err(ErrMode::from_error_kind(&id, ErrorKind::Fail))
    } else {
        Ok(id)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_parse_identifiers() {
        test_parser(
            vec![
                ("01id", Some("01id"), ""),   // Valid prefixed identifier, fully consumed
                ("0id01", Some("0id01"), ""), // Valid suffixed identifier, fully consumed
                ("id191x", Some("id191"), "x"), // Partially valid identifier input, stops before 'x'
                ("1id191x", Some("1id191"), "x"), // Partially valid identifier input, stops before 'x'
                ("", None, ""),                   // Empty input should fail
            ],
            identifier,
        );
    }
}
