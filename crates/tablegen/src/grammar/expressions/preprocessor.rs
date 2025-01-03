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
//! Tablegen preprocessor parsing.
//!

mod chunk;
mod def;

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::AsChar;
use winnow::token::*;
use winnow::*;

use crate::grammar::tokens::helpers::*;


// fn process_conditional<'a>(
//     state: &mut PreprocessorState<'a>,
//     is_ifdef: bool,
//     before: &'a str,
//     name: &'a str,
//     content: &'a str,
// ) -> PResult<()> {
//     state.process_chunk_defines(before)?;
//
//     let mut conditional_state = PreprocessorState::new();
//     conditional_state.process_chunk_defines(content)?;
//
//     state.add_conditional(
//         if is_ifdef { ConditionType::IfDef } else { ConditionType::IfNDef },
//         name,
//         conditional_state.chunks,
//     );
//
//     Ok(())
// }
//
// pub(crate) fn preprocess<'a>(input: &mut &'a str) -> PResult<String> {
//     let mut state = PreprocessorState::new();
//
//     // Parse each conditional directive one at a time
//     while !input.is_empty() {
//         match alt((
//             |i: &mut &'a str| parse_ifdef.map(|(before, (name, content))|
//                 (before, true, name, content)).parse_next(i),
//             |i: &mut &'a str| parse_ifndef.map(|(before, (name, content))|
//                 (before, false, name, content)).parse_next(i)
//         )).parse_next(input) {
//             Ok((before, is_ifdef, name, content)) => {
//                 process_conditional(&mut state, is_ifdef, before, name, content)?;
//             },
//             Err(_) => {
//                 // No more conditional directives found, process remaining text
//                 state.process_chunk_defines(input)?;
//                 break;
//             }
//         }
//     }
//
//     Ok(state.evaluate())
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_chunk_accumulation() {
//         let input = r#"
// initial text
// #define FOO bar
// middle text
// #ifdef FOO
//     conditional text
//     #define BAZ qux
// #endif
// final text"#;
//
//         let mut input_str = input;
//         let mut state = PreprocessorState::new();
//         state.process_chunk_defines(&mut input_str).unwrap();
//
//         assert!(matches!(state.chunks[0], Chunk::Text(_)));
//         assert!(matches!(state.chunks[1], Chunk::Define(_, _)));
//         assert!(matches!(state.chunks[2], Chunk::Text(_)));
//     }
//
//     #[test]
//     fn test_nested_chunks() {
//         let input = r#"
// #define FOO bar
// #ifdef FOO
//     #define BAZ qux
//     content1
//     #ifdef BAZ
//         nested content
//     #endif
// #endif"#;
//
//         let mut input_str = input;
//         let result = preprocess(&mut input_str).unwrap();
//
//         assert!(result.contains("content1"));
//         assert!(result.contains("nested content"));
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use crate::grammar::tokens::helpers::tests::*;
//
//     #[test]
//     fn should_parse_define_preprocessor_directives() {
//         test_parser(
//             vec![
//                 ("#define name", Some(("", "name")), ""), // Valid preprocessor, fully consumed
//                 ("content #define name leftovers", Some(("content ", "name")), "leftovers"), // Partially valid preprocessor input, stops before 'x'
//                 ("", None, ""),                       // Empty input should fail
//             ],
//             parse_defines,
//         );
//     }
// }
