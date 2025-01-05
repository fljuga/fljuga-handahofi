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
//! Tablegen bang operator parsing.
//!

use winnow::combinator::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::PResult;
use winnow::*;

/// Parses bang operators.
pub(crate) fn bang_operator<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded("!", take_while(1.., AsChar::is_alpha)).parse_next(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::grammar::tokens::helpers::tests::*;
}
