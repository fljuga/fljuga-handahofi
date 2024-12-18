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
//! Tablegen preprocessor directives parsing.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

use crate::grammar::tokens::helpers;

fn preprocessor_directive<'a>(input: &mut &'a str) -> PResult<&'a str> {
    fn hash<'a>(input: &mut &'a str) -> PResult<&'a str> {
        literal("#").parse_next(input)
    }

    fn directive<'a>(input: &mut &'a str) -> PResult<&'a str> {
        alt(("define", "ifdef", "ifndef")).parse_next(input)
    }

    helpers::concat(
        [
            hash as helpers::StrParser<'a>,
            directive as helpers::StrParser<'a>,
        ],
        input,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::tests::*;

    #[test]
    fn should_parse_preprocessor_directives() {
        test_parser(
            vec![
                ("#define", Some("#define"), ""), // Valid preprocessor, fully consumed
                ("#definexx", Some("#define"), "xx"), // Partially valid preprocessor input, stops before 'x'
                ("", None, ""),                       // Empty input should fail
            ],
            preprocessor_directive,
        );
    }
}
