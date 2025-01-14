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
//! Tablegen class definitions parsing.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::*;

use crate::grammar::expressions::class_def::opt_value;
use crate::grammar::tokens::*;
use crate::grammar::tokens::helpers::*;
use crate::grammar::tokens::identifier::identifier;
use crate::grammar::tokens::type_name;
use crate::grammar::expressions::values::value;

enum Body<'a> {
    CodeString { name: &'a str, value: &'a str},
    Field { name: &'a str, r#type: &'a str, value: &'a str},
    DefVar { name: &'a str, value: &'a str},
    LetField { name: &'a str, ranges: Ranges, value: &'a str},
}

static EMPTY_RANGE: Ranges = Vec::new();

impl<'a> Body<'a> {
    fn name(&self) -> &str {
        match &self {
            Body::CodeString { name, .. } => name,
            Body::Field { name, .. } => name,
            Body::DefVar { name, .. } => name,
            Body::LetField { name, .. } => name,
        }
    }

    fn r#type(&self) -> &str {
        match &self {
            Body::Field { r#type, .. } => r#type,
            _ => ""
        }
    }

    fn value(&self) -> &str {
        match &self {
            Body::CodeString { value, .. } => value,
            Body::Field { value, .. } => value,
            Body::DefVar { value, .. } => value,
            Body::LetField { value, .. } => value,
        }
    }

    fn ranges(&self) -> &Ranges {
        match &self {
            Body::LetField { ranges, .. } => ranges,
            _ => &EMPTY_RANGE
        }
    }
}

fn parse_let<'a>(input: &mut &'a str) -> PResult<Body<'a>> {
    preceded(spaced_literal("let"), (spaced_parser(identifier), opt_value))
        .map(|(name, value)| Body::LetField {name, ranges: vec![], value})
        .parse_next(input)
}

fn parse_field<'a>(input: &mut &'a str) -> PResult<Body<'a>> {
    (alt((spaced_literal("code"), type_name::type_name)), spaced_parser(identifier))
        .map(|(t, name)| match t {
            "code" => Body::CodeString { name, value: "" },
            _ => Body::Field { name, r#type: t, value: ""}
        })
        .parse_next(input)
}

fn parse_defvar<'a>(input: &mut &'a str) -> PResult<Body<'a>> {
    preceded(spaced_literal("defvar"), separated_pair(spaced_parser(identifier), spaced_literal("="), spaced_parser(value)))
        .map(|(name, value)| Body::LetField {name, ranges: vec![], value})
        .parse_next(input)
}

pub(crate) fn parse_fields<'a>(input: &mut &'a str) -> PResult<Vec<Body<'a>>> {
    separated(0.., alt((
            parse_let,
            parse_field,
            parse_defvar,
        )), spaced_literal(";")).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_parse_let() {
        // test_parser(
        //     vec![
        //         ("class Name {", Some("Name"), ""), // Valid class, fully consumed
        //         ("", None, ""),                     // Empty input should fail
        //     ],
        //     parse_let,
        // );
    }

    #[test]
    fn should_parse_field() {
        // test_parser(
        //     vec![
        //         ("class Name {", Some("Name"), ""), // Valid class, fully consumed
        //         ("", None, ""),                     // Empty input should fail
        //     ],
        //     parse_field,
        // );
    }

    #[test]
    fn should_parse_defvar() {
        // test_parser(
        //     vec![
        //         ("class Name {", Some("Name"), ""), // Valid class, fully consumed
        //         ("", None, ""),                     // Empty input should fail
        //     ],
        //     parse_defvar,
        // );
    }
}
