//! ## fljúga handahófi tablegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! Common Tablegen expressions.
//!

use winnow::ascii::*;
use winnow::combinator::*;
use winnow::error::*;
use winnow::stream::Stream;
use winnow::*;

mod class_def;
mod record_def;
mod values;
mod let_expression;
