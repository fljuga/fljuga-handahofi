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

//! ## fljúga handahófi tablegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! Tablegen token parsing helpers and deconstructed parser plumbing material.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

/// Parses 0..1+
fn bin_digit1<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., ('0', '1')).parse_next(input)
}

/// Parses bin+ or hex+ into signed i64, left for compatibility.
pub(crate) fn hex_or_bin_i64(input: &mut &str) -> PResult<i64> {
    dispatch!(take(2usize);
        "0x" => hex_digit1.try_map(|s| i64::from_str_radix(s, 16)),
        "0b" => bin_digit1.try_map(|s| i64::from_str_radix(s, 2)),
        _ => fail,
    )
    .parse_next(input)
}

/// Parses signed dec number.
fn dec_istr<'a>(input: &mut &'a str) -> PResult<&'a str> {
    (opt(one_of(('+', '-'))), take_while(1.., '0'..='9'))
        .map(|(sign, num): (Option<char>, &str)| match sign {
            Some(_) => {
                let start = num.as_ptr() as usize - input.as_ptr() as usize - 1;
                let end = num.as_ptr() as usize + num.len() - input.as_ptr() as usize;
                assert!(start <= end && end <= input.len(), "Invalid slice range");
                &input[start..end]
            }
            None => num,
        })
        .parse_next(input)
}

/// Parses unsigned dec number.
fn dec_ustr<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., '0'..='9').parse_next(input)
}

// Parses both unsigned and signed dec numbers.
/// Signed dec number parser has higher priority.
pub(crate) fn dec_i64(input: &mut &str) -> PResult<i64> {
    alt((
        dec_istr.try_map(|s: &str| s.parse::<i64>()),
        dec_ustr.try_map(|s: &str| s.parse::<i64>()),
    ))
    .parse_next(input)
}

/// Parses dec hex bin numbers into i64.
pub(crate) fn int(input: &mut &str) -> PResult<i64> {
    alt((dec_i64, hex_or_bin_i64)).parse_next(input)
}

/// Parses identifier alpha+ chars including underscore.
pub(crate) fn alpha_identifier_chars1<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '_')).parse_next(input)
}

/// Parses identifier digit* chars.
pub(crate) fn digit_identifier_chars0<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(0.., '0'..='9').parse_next(input)
}

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

pub(crate) fn identifier<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let id = concat(
        [
            digit_identifier_chars0 as StrParser<'a>,
            alpha_identifier_chars1 as StrParser<'a>,
            digit_identifier_chars0 as StrParser<'a>,
        ],
        input,
    )?;

    if RESERVED_WORDS.contains(&id) {
        Err(ErrMode::from_error_kind(&id, ErrorKind::Fail))
    } else {
        Ok(id)
    }
}

fn type_name_suffix<'a>(input: &mut &'a str) -> PResult<&'a str> {
    literal(">").parse_next(input)
}

pub(crate) fn generic_bits_type_name(input: &mut &str) -> PResult<i64> {
    delimited(literal("bits<"), int, literal(">")).parse_next(input)
}

pub(crate) fn generic_type_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    delimited(literal("type<"), identifier, literal(">")).parse_next(input)
}

pub(crate) fn variable_name_chars<'a>(input: &mut &'a str) -> PResult<&'a str> {
    concat(
        [
            alpha_identifier_chars1 as StrParser<'a>,
            digit_identifier_chars0 as StrParser<'a>,
        ],
        input,
    )
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

    #[test]
    fn should_parse_binary_digits() {
        test_parser(
            vec![
                ("101011", Some("101011"), ""), // Valid binary string, fully consumed
                ("1010x01", Some("1010"), "x01"), // Partially valid input, stops before 'x'
                ("111100001111", Some("111100001111"), ""), // Large valid binary string
                ("", None, ""),                 // Empty input should fail
            ],
            bin_digit1,
        );
    }

    #[test]
    fn should_parse_hex_or_binary_numbers() {
        test_parser(
            vec![
                ("0b101011", Some(43), ""),         // Valid binary string, fully consumed
                ("0b1010x01", Some(10), "x01"),     // Partially valid input, stops before 'x'
                ("0b111100001111", Some(3855), ""), // Large valid binary string
                ("0xACAB", Some(44203), ""),        // Valid hex string, fully consumed
                ("0xACABx01", Some(44203), "x01"),  // Partially valid input, stops before 'x'
                ("0xACAB1111", Some(2896892177), ""), // Large valid hex string
                ("", None, ""),                     // Empty input should fail
            ],
            hex_or_bin_i64,
        );
    }

    #[test]
    fn should_parse_decimal_numbers() {
        test_parser(
            vec![
                ("101", Some(101), ""),                 // Valid dec string, fully consumed
                ("+101x01", Some(101), "x01"), // Partially valid dec input, stops before 'x'
                ("-10194", Some(-10194), ""),  // Large valid negative dec string
                ("-1019412x02", Some(-1019412), "x02"), // Large partially valid negative dec string, stops before 'x'
                ("", None, ""),                         // Empty input should fail
            ],
            dec_i64,
        );
    }

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
