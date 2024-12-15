//! ## fljúga handahófi tablegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! *fljuga-handahofi-tablegen* implements a [winnow] parser for LLVM Tablegen files.
//!

mod tokens;
mod expressions;

#[derive(Debug)]
pub struct TableGen<'a> {
    pub(crate) name: &'a str,
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_parse_numbers() {}
}
