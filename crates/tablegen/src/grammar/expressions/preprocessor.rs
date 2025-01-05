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
//! Tablegen preprocessor chunk.
//!

use crate::grammar::tokens::comments::filter_comments;
use crate::grammar::tokens::helpers::*;
use std::collections::HashSet;
use winnow::combinator::*;
use winnow::error::*;
use winnow::token::*;
use winnow::PResult;
use winnow::*;

#[derive(Debug, Clone, PartialEq)]
enum ConditionType {
    IfDef,
    IfNDef,
}

#[derive(Debug, Clone)]
struct EvalContext {
    defines: HashSet<String>,
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            defines: HashSet::new(),
        }
    }
}

macro_rules! define_parse_cond {
    ($name:ident, $condType:ident, $condition:expr) => {
        pub(crate) fn $name<'a>(input: &mut &'a str) -> PResult<Chunk<'a>> {
            let (name, mut content) = (delimited(
                literal($condition),
                (
                    delimited(
                        space_or_newline1,
                        take_till_space_or_newline,
                        space_or_newline0,
                    ),
                    any_string_terminated_eager(["#endif"]),
                ),
                "#endif",
            ))
            .parse_next(input)?;

            let parsed_content = parse_chunks(&mut content)?;

            Ok(Chunk::Conditional {
                name,
                condition_type: ConditionType::$condType,
                content: parsed_content,
            })
        }
    };
}

define_parse_cond!(parse_ifdef, IfDef, "#ifdef");
define_parse_cond!(parse_ifndef, IfNDef, "#ifndef");

fn parse_define<'a>(input: &mut &'a str) -> PResult<Chunk<'a>> {
    delimited(
        ("#define", space_or_newline1),
        take_till_space_or_newline,
        space_or_newline0,
    )
    .parse_next(input)
    .map(|name| Chunk::Define { name })
}

fn parse_text<'a>(input: &mut &'a str) -> PResult<Chunk<'a>> {
    let text = take_till(1.., |c| c == '#').parse_next(input)?;
    if text.is_empty() {
        Err(ErrMode::Incomplete(Needed::Unknown))
    } else {
        let trimmed_text = text.trim();

        Ok(Chunk::Text(text.trim()))
    }
}

fn parse_chunks<'a>(input: &mut &'a str) -> PResult<Chunks<'a>> {
    let mut chunks = Vec::new();

    while !input.is_empty() {
        let result = alt((parse_text, parse_define, parse_ifdef, parse_ifndef)).parse_next(input);

        match result {
            Ok(Chunk::Text(text)) if text.is_empty() => continue,
            Ok(chunk) => chunks.push(chunk),
            Err(e) => {
                if !chunks.is_empty() {
                    break;
                }

                return Err(e);
            }
        }
    }

    Ok(Chunks { chunks })
}

#[derive(Debug, Clone, PartialEq)]
enum Chunk<'a> {
    Text(&'a str),
    Define {
        name: &'a str,
    },
    Conditional {
        name: &'a str,
        condition_type: ConditionType,
        content: Chunks<'a>,
    },
}

impl<'a> Chunk<'a> {
    fn eval_ctx(&self, ctx: &mut EvalContext) -> String {
        match self {
            Chunk::Text(text) => text.to_string(),
            Chunk::Define { name } => {
                ctx.defines.insert(name.to_string());
                String::new()
            }
            Chunk::Conditional {
                name,
                condition_type,
                content,
            } => {
                let is_defined = ctx.defines.contains(&name.to_string());
                let should_include = match condition_type {
                    ConditionType::IfDef => is_defined,
                    ConditionType::IfNDef => !is_defined,
                };

                if should_include {
                    content.eval_ctx(ctx)
                } else {
                    String::new()
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Chunks<'a> {
    chunks: Vec<Chunk<'a>>,
}

impl<'a> Chunks<'a> {
    fn eval_ctx(&self, ctx: &mut EvalContext) -> String {
        self.chunks
            .iter()
            .map(|chunk| chunk.eval_ctx(ctx))
            .collect()
    }
}

pub(crate) fn preprocess<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let mut ctx = EvalContext::new();
    let mut filtered_comments = filter_comments(input)?;
    let chunks = parse_chunks(&mut filtered_comments)?;
    let result = chunks.eval_ctx(&mut ctx);
    Ok(Box::leak(result.into_boxed_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::grammar::tokens::helpers::tests::*;

    #[test]
    fn should_parse_defines() {
        test_parser(
            vec![
                ("#define NAME", Some(Chunk::Define { name: "NAME" }), ""), // Valid define, fully consumed
                (
                    "#define NAME \nxx",
                    Some(Chunk::Define { name: "NAME" }),
                    "xx",
                ), // Partially valid define input, stops before '\nxx'
                ("#define", None, ""), // Invalid define, consumed
            ],
            parse_define,
        );
    }

    #[test]
    fn should_parse_text() {
        test_parser(
            vec![
                ("any", Some(Chunk::Text("any")), ""), // Valid text, fully consumed
                (
                    "before define \n#define NAME \nxx",
                    Some(Chunk::Text("before define")),
                    "#define NAME \nxx",
                ), // Partially valid text input, stops before #define
                ("#define", None, "#define"),          // Empty text
            ],
            parse_text,
        );
    }

    #[test]
    fn should_parse_ifdef() {
        test_parser(
            vec![
                (
                    "#ifdef NAME \nxx #endif",
                    Some(Chunk::Conditional {
                        name: "NAME",
                        condition_type: ConditionType::IfDef,
                        content: Chunks {
                            chunks: vec![Chunk::Text("xx")],
                        },
                    }),
                    "",
                ),
                (
                    "#ifdef NAME\nsome content\n#endif",
                    Some(Chunk::Conditional {
                        name: "NAME",
                        condition_type: ConditionType::IfDef,
                        content: Chunks {
                            chunks: vec![Chunk::Text("some content")],
                        },
                    }),
                    "",
                ),
                (
                    "#ifdef NAME\n#ifndef NAME2\nsome content\n#endif\n#endif",
                    Some(Chunk::Conditional {
                        name: "NAME",
                        condition_type: ConditionType::IfDef,
                        content: Chunks {
                            chunks: vec![Chunk::Conditional {
                                name: "NAME2",
                                condition_type: ConditionType::IfNDef,
                                content: Chunks {
                                    chunks: vec![Chunk::Text("some content")],
                                },
                            }],
                        },
                    }),
                    "",
                ),
                ("#ifdef", None, ""), // Invalid ifdef, consumed
            ],
            parse_ifdef,
        );
    }

    #[test]
    fn should_preprocess() {
        test_parser(
            vec![
                (
                    "#ifdef NAME\n#ifndef NAME2\nsome content\n#endif\n#endif",
                    Some(""),
                    "",
                ), // Absent define
                (
                    "#define NAME\n#ifdef NAME\n#ifndef NAME2\nsome content\n#endif\n#endif",
                    Some("some content"),
                    "",
                ), // Present define
                (
                    "#define NAME\n#ifdef NAME\ncontent\n#endif",
                    Some("content"),
                    "",
                ), // Define and condition
                (
                    "#define NAME\n#ifdef NAME\n/* multiline */// single-line\ncontent\n#endif",
                    Some("content"),
                    "",
                ), // Define and condition
                (
                    "#define NAME\n#ifdef NAME\n/* multiline */ // single-line\ncontent\n#endif",
                    Some("content"),
                    "",
                ), // Define and condition
            ],
            preprocess,
        );
    }
}
