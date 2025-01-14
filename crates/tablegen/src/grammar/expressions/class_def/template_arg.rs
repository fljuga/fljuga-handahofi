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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ClassTemplateArg<'a> {
    typ: &'a str,
    identifier: &'a str,
    value: &'a str,
}

pub(crate) fn parse_template_args<'a>(input: &mut &'a str) -> PResult<Vec<ClassTemplateArg<'a>>> {
    let pair = (spaced_parser(identifier), spaced_parser(identifier), opt_value).map(|(typ, id, value)| ClassTemplateArg { typ, identifier: id, value });

    let pairs = separated(1.., pair, spaced_literal(","));

    delimited(spaced_literal("<"), pairs, spaced_literal(">")).parse_next(input)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_parse_template_args() {
        test_parser(
            vec![
                ("<A B, C \nD, \n E F =  \n val\n>", Some(vec![
                    ClassTemplateArg { typ: "A", identifier: "B", value: "?" },
                    ClassTemplateArg { typ: "C", identifier: "D", value: "?" },
                    ClassTemplateArg { typ: "E", identifier: "F", value: "val" }]
                ), ""), // Valid class, fully consumed
                // ("", None, ""),                     // Empty input should fail
            ],
            parse_template_args,
        );
    }

    #[test]
    fn should_parse_class_def() {
        // test_parser(
        //     vec![
        //         ("class Name {", Some("Name"), ""), // Valid class, fully consumed
        //         ("", None, ""),                     // Empty input should fail
        //     ],
        //     class_def,
        // );
    }
}
