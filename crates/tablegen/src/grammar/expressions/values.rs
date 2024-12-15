use winnow::ascii::*;
use winnow::combinator::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::error::*;
use winnow::stream::{AsChar, Stream};
use winnow::token::*;
use winnow::PResult;
use winnow::*;

use crate::grammar::tokens::internal;

fn value<'a>(input: &mut &'a str) -> PResult<&'a str> {
    simple_value.parse_next(input)
    // (
    //     simple_value,
    //     // repeat(0.., value_suffix),
    // ).parse_next(input)
    // // .map(|(a, b): (&str, Vec<&str>)| a)
}

fn simple_value<'a>(input: &mut &'a str) -> PResult<&'a str> {
    "x".parse_next(input)
}

fn value_suffix(input: &mut &str) -> PResult<Vec<(i64, i64)>> {
    delimited("{", range_list, "}").parse_next(input)
}

fn range_list(input: &mut &str) -> PResult<Vec<(i64, i64)>> {
    repeat(1.., range_int_piece).parse_next(input)
}

fn range_int_piece(input: &mut &str) -> PResult<(i64, i64)> {
    alt((
        (internal::int, internal::spaced("..."), internal::int),
        (internal::int, take_while(1.., AsChar::is_space), internal::int),
        (internal::int, internal::spaced("-"), internal::int),
    )).map(|(a, _, b)| (a, b)).parse_next(input)
}

fn range_value_piece<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str)> {
    alt((
        (value, internal::spaced("..."), value),
        (value, take_while(1.., AsChar::is_space), value),
        (value, internal::spaced("-"), value),
    )).map(|(a, _, b)| (a, b)).parse_next(input)
}

// fn slice_elements<'a>(input: &mut &'a str) -> PResult<&'a str> {
//     delimited("(", slice_element, ")").parse_next(input)
// }

// fn slice_element<'a>(input: &mut &'a str) -> PResult<&'a str> {
//     range_value_piece.parse_next(input)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use internal::tests::*;

    #[test]
    fn should_parse_ranges() {
        let output56 = Some((5, 6));
        test_parser(
            vec![
                ("5 ... 6", output56, ""),    // Valid range, fully consumed
                ("5...6xx", output56, "xx"),  // Partially valid range input, stops before 'x'
                ("5-6", output56, ""),    // Valid range, fully consumed
                ("5 -6", Some((5, -6)), ""),  // Invalid range, fully consumed
                ("5 6xx", output56, "xx"),  // Partially valid range input, stops before 'x'
                ("5 6", output56, ""),    // Valid range, fully consumed
                ("5   6xx", output56, "xx"),  // Partially valid range input, stops before 'x'
                ("5   -6xx", Some((5, -6)), "xx"),  // Partially valid range input with neg exclusion, stops before 'x'\
                ("5--6xx", Some((5,-6)), "xx"),  // Partially valid range input with neg exclusion, stops before 'x'
                ("", None, ""),                   // Empty input should fail
            ],
            range_int_piece,
        );
    }
}
