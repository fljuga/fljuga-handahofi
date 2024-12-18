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
//! Tablegen ranges parsing.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

use crate::grammar::tokens::helpers::*;
use crate::grammar::tokens::digits::*;

use crate::grammar::expressions::values::*;

pub(crate) fn range_list(input: &mut &str) -> PResult<Vec<(i64, i64)>> {
    repeat(1.., range_int_piece).parse_next(input)
}

pub(crate) fn range_int_piece(input: &mut &str) -> PResult<(i64, i64)> {
    alt((
        (int, spaced("..."), int),
        (
            int,
            take_while(1.., AsChar::is_space),
            int,
        ),
        (int, spaced("-"), int),
    ))
        .map(|(a, _, b)| (a, b))
        .parse_next(input)
}

pub(crate) fn range_value_piece<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str)> {
    alt((
        (value, spaced("..."), value),
        (value, take_while(1.., AsChar::is_space), value),
        (value, spaced("-"), value),
    ))
        .map(|(a, _, b)| (a, b))
        .parse_next(input)
}
