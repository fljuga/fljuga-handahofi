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

//! ## fljúga handahófi tablegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! *fljuga-handahofi-tablegen* implements a [winnow] parser for LLVM Tablegen files.
//!

use winnow::PResult;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::error::*;
use winnow::stream::{AsChar, Stream};
use winnow::token::*;
use winnow::*;

mod expressions;
mod tokens;

#[derive(Debug)]
pub struct TableGen<'a> {
    pub(crate) name: &'a str,
}

// fn range_value_piece<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str)> {
//     alt((
//         (crate::grammar::expressions::values::value, internal::spaced("..."), crate::grammar::expressions::values::value),
//     ))
//         .map(|(a, _, b)| (a, b))
//         .parse_next(input)
// }

#[cfg(test)]
mod tests {
    use super::*;
}
