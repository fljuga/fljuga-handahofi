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
//! Tablegen token parsing helpers and plumbing material.
//!

use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::PResult;
use winnow::*;

pub type GenParserPtr<'a, T> = fn(&mut &'a str) -> PResult<&'a T>;

/// Type alias for parser functions that work on strings.
pub type StrParser<'a> = GenParserPtr<'a, str>;

/// Concatenates a series of parsers into one that matches their combined input.
///
/// # Details:
/// Each parser matches a portion of the input string sequentially. This function returns
/// a string slice starting at the beginning of the input and ending at the last matched token.
pub(crate) fn concat<'a, P, const N: usize>(
    parsers: [P; N],
) -> impl Fn(&mut &'a str) -> PResult<&'a str>
where
    P: Fn(&mut &'a str) -> PResult<&'a str>,
{
    move |input: &mut &'a str| {
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
}

/// Creates a parser for a static string surrounded by optional spaces or newlines.
pub(crate) fn spaced_literal<'a>(literal_str: &'static str) -> impl Fn(&mut &'a str) -> PResult<&'a str> {
    move |input: &mut &'a str| {
        delimited(space_or_newline0, literal(literal_str), space_or_newline0).parse_next(input)
    }
}

pub(crate) fn spaced_parser<'a, F>(parser: F) -> impl Fn(&mut &'a str) -> PResult<&'a str>
where
    F: Fn(&mut &'a str) -> PResult<&'a str> + Clone
{
    move |input: &mut &'a str| {
        delimited(space_or_newline0, parser.clone(), space_or_newline0).parse_next(input)
    }
}

#[cfg_attr(test, mutants::skip)]
/// Parses a substring until the last specified terminator is encountered, then consumes it skipping the terminator.
fn any_string_terminated<'a, const N: usize>(
    endings: [&'static str; N],
    eager: bool,
) -> impl Fn(&mut &'a str) -> PResult<&'a str> {
    move |input: &mut &'a str| {
        let mut end_pos = if eager { 0 } else { input.len() };
        let mut found = false;

        for ending in &endings {
            if let Some(pos) = if eager {
                input.rfind(ending)
            } else {
                input.find(ending)
            } {
                if eager && pos > end_pos || !eager && pos < end_pos {
                    end_pos = pos;
                }
                found = true;
            }
        }

        if !found {
            let matched = *input;
            *input = "";
            return if matched.is_empty() {
                Err(ErrMode::Incomplete(Needed::Unknown))
            } else {
                Ok(matched)
            };
        }

        let (matched, remainder) = input.split_at(end_pos);
        *input = remainder;
        if matched.is_empty() {
            Err(ErrMode::Incomplete(Needed::Unknown))
        } else {
            Ok(matched)
        }
    }
}

/// Parses a substring lazily until the last specified terminator is encountered, then consumes it optionally skipping the terminator.
pub(crate) fn any_string_terminated_lazy<'a, const N: usize>(
    endings: [&'static str; N],
) -> impl Fn(&mut &'a str) -> PResult<&'a str> {
    any_string_terminated(endings, false)
}

/// Parses a substring eagerly until the last specified terminator is encountered, then consumes it optionally skipping the terminator.
/// Eager parsing does not work for chained terminators.
pub(crate) fn any_string_terminated_eager<'a, const N: usize>(
    endings: [&'static str; N],
) -> impl Fn(&mut &'a str) -> PResult<&'a str> {
    any_string_terminated(endings, true)
}

pub(crate) fn any_string<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_till(1.., |_| false).parse_next(input)
}

pub(crate) fn take_till_space_or_newline<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_till(1.., |c| AsChar::is_newline(c) || AsChar::is_space(c)).parse_next(input)
}

pub(crate) fn space_or_newline0<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(0.., |c| AsChar::is_newline(c) || AsChar::is_space(c)).parse_next(input)
}

pub(crate) fn space_or_newline1<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., |c| AsChar::is_newline(c) || AsChar::is_space(c)).parse_next(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Shared testing utility.
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

    #[test]
    fn should_parse_any_string() {
        test_parser(
            vec![
                ("ab", Some("ab"), ""),     // Valid string, fully consumed
                (" \n ", Some(" \n "), ""), // Valid string of space chars, fully consumed
                ("", None, ""),             // Empty input should fail
            ],
            any_string,
        );
    }

    #[test]
    fn should_parse_any_string_terminated_eager() {
        test_parser(
            vec![
                ("ab", Some("ab"), ""),         // Valid non-terminated string, fully consumed
                ("ab##", Some("ab"), "##"),     // Valid string, fully consumed
                (" \n## ", Some(" \n"), "## "), // Valid string of space chars
                (" \n! abc ##  !", Some(" \n! abc ##  "), "!"), // Valid string of space chars, fully consumed
                ("##", None, "##"), // Empty terminated input should fail
                ("", None, ""),     // Empty input should fail
            ],
            any_string_terminated_eager(["##", "!"]),
        );
    }
    #[test]
    fn should_parse_any_string_terminated_lazy() {
        test_parser(
            vec![
                ("ab", Some("ab"), ""),         // Valid non-terminated string, fully consumed
                ("ab##", Some("ab"), "##"),     // Valid string, fully consumed
                (" \n## ", Some(" \n"), "## "), // Valid string of space chars
                (" \n! abc ##  !", Some(" \n"), "! abc ##  !"), // Valid string of space chars, fully consumed
                ("!", None, "!"),     // Empty terminated input should fail
                ("##", None, "##"),   // Empty terminated input should fail
                ("##!", None, "##!"), // Empty terminated input should fail
                ("", None, ""),       // Empty input should fail
            ],
            any_string_terminated_lazy(["##", "!"]),
        );
    }

    #[test]
    fn should_concatenate_parsers() {
        test_parser(
            vec![
                ("ab", Some("ab"), ""),     // Valid concatenation, fully consumed
                ("abxx", Some("ab"), "xx"), // Partially valid concatenation
                ("", None, ""),             // Empty input should fail
            ],
            concat([
                (|input: &mut &str| literal("a").parse_next(input)) as StrParser,
                |input: &mut &str| literal("b").parse_next(input),
            ]),
        );
    }
}
