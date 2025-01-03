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
//! ## fljúga handahófi mlir codegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! *fljuga-handahofi-mlir-codegen* generates rust bindings for [mlir-c](https://mlir.llvm.org/docs/CAPI/) API using LLVM TableGen format.
//! Implements a subset of TableGen format parsing via [fljuga_handahofi_tablegen] for rust **mlir-c** binging code generation.
//! Does not rely on LLVM `*-tblgen` tools or direct API wrapping.
//!
//! *fljuga-handahofi-mlir-codegen* mirrors the existing [MLIR Python bindings](https://github.com/llvm/llvm-project/tree/main/mlir/python/mlir/dialects),
//! targetting features of ML-related python framewokrs, like [JAX Pallas](https://jax.readthedocs.io/en/latest/pallas/index.html),
//! [Torch-MLIR](https://github.com/llvm/torch-mlir) and similar compilation backends [Triton](https://github.com/triton-lang/triton), [XLA](https://openxla.org/xla).
//!

#![feature(async_closure)]
extern crate fljuga_handahofi_tablegen;

mod client;
mod tablegen;
