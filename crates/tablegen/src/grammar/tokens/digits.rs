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
//! Tablegen digits parsing.
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


#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::grammar::tokens::helpers::tests::*;

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
}