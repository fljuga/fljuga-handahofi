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

use std::borrow::Cow;
use winnow::combinator::*;
use winnow::PResult;
use winnow::*;

fn filter_terminated<'a>(start: &str, end: &str, strip_ending: bool, input: &mut &'a str) -> PResult<Cow<'a, str>> {
    let mut working_copy = Cow::Borrowed(*input);

    while let Some(start_pos) = working_copy.find(start) {
        let end_pos = start_pos + working_copy[start_pos..].find(end).unwrap_or(working_copy.len() - start_pos);
        let after = if strip_ending {
            Cow::Borrowed(&working_copy[end_pos + end.len()..])
        } else {
            Cow::Borrowed(&working_copy[end_pos..])
        };

        let mut new_working_copy = String::with_capacity(start_pos + after.len());
        new_working_copy.push_str(&working_copy[..start_pos]);
        new_working_copy.push_str(&after);

        working_copy = Cow::Owned(new_working_copy);
    }

    Ok(working_copy)
}

fn filter_single_line<'a>(input: &mut &'a str) -> PResult<Cow<'a, str>> {
    let result = filter_terminated("//", "\n", false, input);
    *input = "";
    result
}

fn filter_multi_line<'a>(input: &mut &'a str) -> PResult<Cow<'a, str>> {
    let result = filter_terminated("/*", "*/", true, input);
    *input = "";
    result
}

pub(crate) fn filter_comments<'a>(input: &mut &'a str) -> PResult<Cow<'a, str>> {
    let result = match filter_multi_line(input)? {
        Cow::Borrowed(input) => filter_single_line(&mut &*input),
        Cow::Owned(input) => Ok(Cow::Owned(filter_single_line(&mut &*input)?.into_owned())),
    };
    *input = "";
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_filter_single_line_comment() {
        test_parser(
            vec![
                ("anything // comment\nabc", Some(Cow::from("anything \nabc")), ""), // Valid comment, terminated with \n
                // ("anything // comment\n", Some(Cow::from("anything \n")), ""), // Valid comment, terminated with \n, fully consumed
                // ("anything // comment", Some(Cow::from("anything \n")), ""), // Valid comment, unterminated, fully consumed
                (
                    "anything /* comment\nmore\nlines\n */ // single-line\n",
                    Some(Cow::from("anything /* comment\nmore\nlines\n */ \n")),
                    "",
                ), // Valid comment, terminated with chained multi-line /**/, fully consumed
                ("", Some(Cow::from("")), ""),                                   // Empty input should not fail
            ],
            filter_single_line,
        );
    }

    #[test]
    fn should_parse_multi_line_comment() {
        test_parser(
            vec![
                (
                    "anything /* comment\nmore\nlines\n */\nabc\n",
                    Some(Cow::from("anything \nabc\n")),
                    "",
                ), // Valid comment, terminated with */
                (
                    "anything /* comment\nmore\nlines\n */",
                    Some(Cow::from("anything ")),
                    "",
                ), // Valid comment, terminated with */, fully consumed
                (
                    "anything /* comment\nmore\nlines\n *///",
                    Some(Cow::from("anything //")),
                    "",
                ), // Valid comment, terminated with */, fully consumed
                ("", Some(Cow::from("")), ""),                         // Empty input should not fail
            ],
            filter_multi_line,
        );
    }

    #[test]
    fn should_filter_comments() {
        test_parser(
            vec![
                ("code // comment\nmore code", Some(Cow::from("code \nmore code")), ""), // Single line comment
                ("code // comment\n", Some(Cow::from("code \n")), ""), // Single line comment, fully consumed
                ("code /* comment */ more code", Some(Cow::from("code  more code")), ""), // Multi-line comment
                ("code /* comment */ ", Some(Cow::from("code  ")), ""), // Multi-line comment, fully consumed
                (
                    "code // single line\nmore code /* \nmulti-line\n */ end",
                    Some(Cow::from("code \nmore code  end")),
                    "",
                ), // Mixed comments
                (
                    "code /* multi-line */// single line\nend",
                    Some(Cow::from("code \nend")),
                    "",
                ), // Mixed comments, fully consumed
                ("code with no comments", Some(Cow::from("code with no comments")), ""), // No comments
                ("", Some(Cow::from("")), ""),                         // Empty input should not fail
            ],
            filter_comments,
        );
    }
}
