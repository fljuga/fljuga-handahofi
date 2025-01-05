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
//! Tablegen comments parsing.
//!

use winnow::combinator::*;
use winnow::token::*;
use winnow::PResult;
use winnow::*;

use crate::grammar::tokens::helpers::*;

fn single_line_comment<'a>(input: &mut &'a str) -> PResult<&'a str> {
    if let Some(_) = input.find("///") {
        // chained comments hack ///**/
        *input = Box::leak::<'a>(input.replace("///", "/ //").into_boxed_str());
    }

    let result = terminated(
        any_string_terminated_lazy(["//"]),
        ("//", any_string_terminated_lazy(["\n"]), opt("\n")),
    )
    .parse_next(input)?;
    Ok(Box::leak(format!("{}\n", result).into_boxed_str()))
}

fn multi_line_comment<'a>(input: &mut &'a str) -> PResult<&'a str> {
    terminated(
        any_string_terminated_lazy(["/*"]),
        delimited("/*", take_until(0.., "*/"), "*/"),
    )
    .parse_next(input)
}

pub(crate) fn filter_comments<'a>(input: &mut &'a str) -> PResult<&'a str> {
    while input.contains("//") || input.contains("/*") {
        match alt((
            single_line_comment,
            multi_line_comment,
            any_string_terminated_lazy(["//", "/*"]),
        ))
        .parse_next(input)
        {
            Ok(filtered) => {
                *input = Box::leak::<'a>(format!("{}{}", filtered, input).into_boxed_str());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    let res = *input;
    *input = "";

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_parse_any_string() {
        test_parser(
            vec![
                ("anything ##", Some("anything "), "##"), // Valid string, terminated with #
                ("", None, ""),                           // Empty input should fail
            ],
            any_string_terminated_lazy(["##"]),
        );
    }

    #[test]
    fn should_parse_single_line_comment() {
        test_parser(
            vec![
                ("anything // comment\nabc", Some("anything \n"), "abc"), // Valid comment, terminated with \n
                ("anything // comment\n", Some("anything \n"), ""), // Valid comment, terminated with \n, fully consumed
                ("anything // comment", Some("anything \n"), ""), // Valid comment, unterminated, fully consumed
                (
                    "anything /* comment\nmore\nlines\n */// single-line\n",
                    Some("anything /* comment\nmore\nlines\n */ \n"),
                    "",
                ), // Valid comment, terminated with chained multi-line /**/, fully consumed
                ("", None, ""),                                   // Empty input should fail
            ],
            single_line_comment,
        );
    }

    #[test]
    fn should_parse_multi_line_comment() {
        test_parser(
            vec![
                (
                    "anything /* comment\nmore\nlines\n */\nabc\n",
                    Some("anything "),
                    "\nabc\n",
                ), // Valid comment, terminated with */
                (
                    "anything /* comment\nmore\nlines\n */",
                    Some("anything "),
                    "",
                ), // Valid comment, terminated with */, fully consumed
                (
                    "anything /* comment\nmore\nlines\n *///",
                    Some("anything "),
                    "//",
                ), // Valid comment, terminated with */, fully consumed
                ("", None, ""), // Empty input should fail
            ],
            multi_line_comment,
        );
    }

    #[test]
    fn should_filter_comments() {
        test_parser(
            vec![
                ("code // comment\nmore code", Some("code \nmore code"), ""), // Single line comment
                ("code // comment\n", Some("code \n"), ""), // Single line comment, fully consumed
                ("code /* comment */ more code", Some("code  more code"), ""), // Multi-line comment
                ("code /* comment */ ", Some("code  "), ""), // Multi-line comment, fully consumed
                (
                    "code // single line\nmore code /* \nmulti-line\n */ end",
                    Some("code \nmore code  end"),
                    "",
                ), // Mixed comments
                (
                    "code /* multi-line */ x // single line\nend",
                    Some("code  x \nend"),
                    "",
                ), // Mixed comments, fully consumed
                ("code with no comments", Some("code with no comments"), ""), // No comments
                ("", Some(""), ""),                         // Empty input should fail
            ],
            filter_comments,
        );
    }
}
