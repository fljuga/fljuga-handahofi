//! ## fljúga handahófi mlir codegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//! MLIR itself supports numerous heterogeneous compilation dialects targeting variety of devices: CPUs, GPUs, TPUs.
//! It even targets hardware description languages with [circt](https://github.com/llvm/circt) project.
//!

mod targets;