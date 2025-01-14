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

mod body;
mod template_arg;

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::*;
use crate::grammar::expressions::class_def::template_arg::ClassTemplateArg;
use crate::grammar::tokens::*;
use crate::grammar::tokens::helpers::*;
use crate::grammar::tokens::identifier::identifier;
use crate::grammar::tokens::type_name;
use crate::grammar::expressions::values::value;

#[derive(Debug, Clone, PartialEq)]
struct ClassDefinition<'a> {
    name: &'a str,
    template_arguments: Vec<ClassTemplateArg<'a>>,
}

struct ParentClassDefinition<'a> {
    name: &'a str,
    arguments: Vec<ClassTemplateArg<'a>>,
}

// pub(crate) fn parse_class<'a>(input: &mut &'a str) -> PResult<&'a str> {
//     preceded(("class", space_or_newline1), (identifier, delimited(("<", space_or_newline1), literal, (space_or_newline1, ">")))).parse_next(input)
// }

fn opt_value<'a>(input: &mut &'a str) -> PResult<&'a str> {
    opt(preceded(opt(spaced_literal("=")), spaced_parser(identifier))).map(|opt| opt.unwrap_or_else(|| "")).parse_next(input)
}


// fn class_def<'a>(input: &mut &'a str) -> PResult<ClassDefinition<'a>> {
//
//     let body = delimited(spaced_literal("{"), (), spaced_literal("}"));
//
//     preceded(spaced_literal("class"), (spaced_parser(identifier), spaced_parser(parse_template_args), body)).parse_next(input)
// }

// fn class_property<'a>(input: &mut &'a str) -> PResult<ClassProperty<'a>> {
//     "x".map(|_| ClassProperty{name: "x", ty: "x"}).parse_next(input)
// }

#[cfg(test)]
mod tests {
    use super::*;
}
