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
//! Tablegen token parsing helpers and plumbing material.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

pub(crate) fn concat<'a, P, const N: usize>(
    parsers: [P; N],
    input: &mut &'a str,
) -> PResult<&'a str>
where
    P: Fn(&mut &'a str) -> PResult<&'a str>,
{
    // Clone the original input slice to calculate offsets correctly
    let original_input = *input;
    let original_start_ptr = original_input.as_ptr() as usize;

    let mut last_end_ptr = original_start_ptr;

    for parser in parsers.iter() {
        let parsed = parser(input)?;
        last_end_ptr = parsed.as_ptr() as usize + parsed.len();
    }

    Ok(&original_input[0..last_end_ptr - original_start_ptr])
}

pub type GenParser<T> = fn(&mut &str) -> PResult<T>;

pub type GenParserPtr<'a, T> = fn(&mut &'a str) -> PResult<&'a T>;

pub type StrParser<'a> = GenParserPtr<'a, str>;

pub(crate) fn spaced<'a>(s: &'static str) -> impl Fn(&mut &'a str) -> PResult<&'a str> {
    move |input: &mut &'a str| delimited(space0, literal(s), space0).parse_next(input)
}

pub(crate) fn any_string<'a>(terminated: &'static str) -> impl Fn(&mut &'a str) -> PResult<&'a str> {
    move |input: &mut &'a str| {
        take_until(0.., terminated).parse_next(input)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub(crate) fn test_parser<'a, T, P>(cases: Vec<(&'a str, Option<T>, &'a str)>, parser: P)
    where
        P: Fn(&mut &'a str) -> PResult<T>,
        T: PartialEq + std::fmt::Debug,
    {
        for (input, expected_result, expected_remainder) in cases {
            let mut input = input; // Create mutable reference for the parser

            match (parser(&mut input), expected_result) {
                (Ok(parsed), Some(expected)) => {
                    assert_eq!(
                        parsed, expected,
                        "Expected '{:?}', but got '{:?}'",
                        expected, parsed
                    );
                    assert_eq!(
                        input, expected_remainder,
                        "Expected remainder '{}', but got '{}'",
                        expected_remainder, input
                    );
                }
                (Err(_), None) => {
                    assert_eq!(
                        input, expected_remainder,
                        "Expected parser to fail with remainder '{}', but got '{}'",
                        expected_remainder, input
                    );
                }
                (result, _) => panic!("Unexpected result: {:?}", result),
            }
        }
    }
}
