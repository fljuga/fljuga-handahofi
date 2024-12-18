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
//! Tablegen variables parsing.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

use crate::grammar::tokens::helpers::*;
use crate::grammar::tokens::identifier::*;

pub(crate) fn variable_name_chars<'a>(input: &mut &'a str) -> PResult<&'a str> {
    concat(
        [
            alpha_identifier_chars1 as StrParser<'a>,
            digit_identifier_chars0 as StrParser<'a>,
        ],
        input,
    )
}

fn variable_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded('$', variable_name_chars).parse_next(input)
}



#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::grammar::tokens::helpers::tests::*;
}