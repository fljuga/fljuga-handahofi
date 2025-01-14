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

//! ## fljúga handahófi tablegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! Common Tablegen tokens.
//!

pub(crate) mod bang_operator;

pub(crate) mod comments;

pub(crate) mod digits;

pub(crate) mod helpers;

pub(crate) mod identifier;

pub(crate) mod strings;

pub(crate) mod type_name;

pub(crate) mod variable;
pub(crate) mod ranges;

pub(crate) type Range = std::ops::Range<i64>;

pub(crate) type Ranges = Vec<Range>;

#[cfg(test)]
mod tests {}
