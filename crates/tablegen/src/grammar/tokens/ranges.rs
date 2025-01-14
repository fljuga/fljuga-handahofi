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
//! Tablegen ranges parsing.
//!

use std::ops::RangeFrom;
use winnow::combinator::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::PResult;
use winnow::*;

use crate::grammar::tokens::*;
use crate::grammar::tokens::digits::*;
use crate::grammar::tokens::helpers::*;


fn ranged_parser<'a, F, T>(parser: F) -> impl Fn(&mut &'a str) -> PResult<(T, T)>
where
    F: Fn(&mut &'a str) -> PResult<T> + Clone
{
    move |input: &mut &'a str| {
        alt((
            (parser.clone(), spaced_literal("..."), parser.clone()),
            (parser.clone(), take_while(1.., AsChar::is_space), parser.clone()),
            (parser.clone(), spaced_literal("-"), parser.clone()),
        ))
            .map(|(a, _, b)| (a, b))
            .parse_next(input)
    }
}

pub(crate) fn range_list(input: &mut &str) -> PResult<Vec<Range>> {
    separated(1.., ranged_parser(int).map(|(from, to)| std::ops::Range { start: from, end: to}), ",")
        .parse_next(input)
}
