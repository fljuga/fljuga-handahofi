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
//! Tablegen comments parsing.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

use crate::grammar::tokens::helpers::*;


fn single_line_comment<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded("//", any_string("\n")).parse_next(input)
}

fn multi_line_comment<'a>(input: &mut &'a str) -> PResult<&'a str> {
    delimited(
        "/*",
        take_until(0.., "*/"),
        "*/",
    ).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_parse_any_string() {
        test_parser(
            vec![
                ("anything ##", Some("anything "), "##"), // Valid string, terminated with #
                ("", None, ""),                       // Empty input should fail
            ],
            any_string("##")
        );
    }

    #[test]
    fn should_parse_single_line_comment() {}

    #[test]
    fn should_parse_multi_line_comment() {}
}