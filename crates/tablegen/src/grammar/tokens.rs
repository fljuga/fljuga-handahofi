//! ## fljúga handahófi tablegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! Common Tablegen tokens.
//!

use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::{AsChar, Stream};
use winnow::token::*;
use winnow::PResult;
use winnow::*;

pub(crate) mod internal;

fn string<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded('"', terminated(take_while(1.., |c: char| c != '"'), '"')).parse_next(input)
}

fn code<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded(
        "[{",
        terminated(take_while(1.., |c: char| !"[{}]".contains(c)), "}]"),
    )
        .parse_next(input)
}

fn variable_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded('$', internal::variable_name_chars).parse_next(input)
}

/// Parses bang operators.
pub(crate) fn bang_operator<'a>(input: &mut &'a str) -> PResult<&'a str> {
    preceded("!", take_while(1.., AsChar::is_alpha)).parse_next(input)
}

fn preprocessor_directive<'a>(input: &mut &'a str) -> PResult<&'a str> {
    fn hash<'a>(input: &mut &'a str) -> PResult<&'a str> {
        literal("#").parse_next(input)
    }

    fn directive<'a>(input: &mut &'a str) -> PResult<&'a str> {
        alt(("define", "ifdef", "ifndef")).parse_next(input)
    }

    internal::concat([
                         hash as internal::StrParser<'a>,
                         directive as internal::StrParser<'a>,
                     ], input)
}

fn type_name<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        "bit",
        "int",
        "string",
        "dag",
        // internal::generic_bits_type_name,
        // internal::generic_type_name,
        internal::identifier,
    )).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal::tests::*;

    #[test]
    fn should_parse_preprocessor_directives() {
        test_parser(
            vec![
                ("#define", Some("#define"), ""),      // Valid preprocessor, fully consumed
                ("#definexx", Some("#define"), "xx"),  // Partially valid preprocessor input, stops before 'x'
                ("", None, ""),                   // Empty input should fail
            ],
            preprocessor_directive,
        );
    }
}
