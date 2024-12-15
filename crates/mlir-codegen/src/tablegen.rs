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
//! ## fljúga handahófi mlir codegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! *fljuga-handahofi-mlir-codegen* generates rust bindings for [mlir-c](https://mlir.llvm.org/docs/CAPI/) API using LLVM TableGen format.
//!
//! TableGen module processes and code generates various MLIR dialect bindings.
//! *fljúga handahófi* targets the majority of MLIR dialects.
//!

use crate::client::ClientError;
use std::path::Path;
use tokio::io::{self, ErrorKind};

/// Derived [thiserror::Error] for TableGen errors
#[derive(thiserror::Error, Debug)]
pub enum TableGenError {
    #[error("{0}")]
    HttpClientError(#[from] ClientError),

    #[error("{0}")]
    IoError(#[from] io::Error),
}

/// TABLE_GEN_FILES contains all the llvm `*.td` files to parse
/// from the [MLIR Python binding](https://github.com/llvm/llvm-project/tree/main/mlir/python/mlir/dialects).
///
/// Should be updated once in a while.
///
const TABLE_GEN_FILES: [&str; 49] = [
    "Dialect/Affine/IR/AffineOps.td",                 // AffineOps.td
    "Dialect/AMDGPU/IR/AMDGPU.td",                    // AMDGPUOps.td
    "Dialect/Arith/IR/ArithOps.td",                   // ArithOps.td
    "Dialect/Async/IR/AsyncOps.td",                   // AsyncOps.td
    "Dialect/Bufferization/IR/BufferizationEnums.td", // BufferizationEnums.td
    "Dialect/Bufferization/IR/BufferizationOps.td",   // BufferizationOps.td
    "Dialect/Bufferization/TransformOps/BufferizationTransformOps.td", // BufferizationTransformOps.td
    "IR/BuiltinOps.td",                                                // BuiltinOps.td
    "Dialect/Complex/IR/ComplexOps.td",                                // ComplexOps.td
    "Dialect/ControlFlow/IR/ControlFlowOps.td",                        // ControlFlowOps.td
    "Dialect/Func/IR/FuncOps.td",                                      // FuncOps.td
    "Dialect/GPU/IR/GPUOps.td",                                        // GPUOps.td
    "Dialect/GPU/TransformOps/GPUTransformOps.td",                     // GPUTransformOps.td
    "Dialect/Index/IR/IndexOps.td",                                    // IndexOps.td
    "Dialect/Linalg/IR/LinalgOps.td",                                  // LinalgOps.td
    "Dialect/Linalg/IR/LinalgStructuredOps.td",                        // LinalgOps.td
    "Dialect/Linalg/TransformOps/LinalgTransformEnums.td", // LinalgStructuredTransformEnums.td
    "Dialect/Linalg/TransformOps/LinalgTransformOps.td",   // LinalgStructuredTransformOps.td
    "Dialect/LLVMIR/LLVMOps.td",                           // LLVMOps.td
    "Dialect/LLVMIR/LLVMIntrinsicOps.td",                  // LLVMOps.td
    "Dialect/Math/IR/MathOps.td",                          // MathOps.td
    "Dialect/MemRef/IR/MemRefOps.td",                      // MemRefOps.td
    "Dialect/MemRef/TransformOps/MemRefTransformOps.td",   // MemRefTransformOps.td
    "Dialect/MLProgram/IR/MLProgramOps.td",                // MLProgramOps.td
    "Dialect/NVGPU/IR/NVGPU.td",                           // NVGPUOps.td
    "Dialect/NVGPU/TransformOps/NVGPUTransformOps.td",     // NVGPUTransformOps.td
    "Dialect/LLVMIR/NVVMOps.td",                           // NVVMOps.td
    "Dialect/OpenMP/OpenMPOps.td",                         // OpenMPOps.td
    "Dialect/PDL/IR/PDLOps.td",                            // PDLOps.td
    "Dialect/LLVMIR/ROCDLOps.td",                          // ROCDLOps.td
    "Dialect/SCF/TransformOps/SCFTransformOps.td",         // SCFLoopTransformOps.td
    "Dialect/Transform/LoopExtension/LoopExtensionOps.td", // SCFLoopTransformOps.td
    "Dialect/SCF/IR/SCFOps.td",                            // SCFOps.td
    "Dialect/Shape/IR/ShapeOps.td",                        // ShapeOps.td
    "Dialect/SparseTensor/IR/SparseTensorAttrDefs.td",     // SparseTensorAttrDefs.td
    "Dialect/SparseTensor/IR/SparseTensorOps.td",          // SparseTensorOps.td
    "Dialect/SparseTensor/TransformOps/SparseTensorTransformOps.td", // "SparseTensorTransformOps.td
    "Dialect/SPIRV/IR/SPIRVOps.td",                        // SPIRVOps.td
    "Dialect/Tensor/IR/TensorOps.td",                      // TensorOps.td
    "Dialect/Tensor/TransformOps/TensorTransformOps.td",   // TensorTransformOps.td
    "Dialect/Tosa/IR/TosaOps.td",                          // TosaOps.td
    "Dialect/Transform/IR/TransformAttrs.td",              // TransformAttrs.td
    "Dialect/Transform/IR/TransformOps.td",                // TransformOps.td
    "Dialect/Transform/PDLExtension/PDLExtensionOps.td",   // TransformPDLExtensionOps.td
    "Dialect/Vector/IR/VectorAttributes.td",               // VectorAttributes.td
    "Dialect/Vector/IR/VectorOps.td",                      // VectorOps.td
    "Dialect/Vector/IR/Vector.td",                         // Vector.td
    "Dialect/Vector/TransformOps/VectorTransformOps.td",   // VectorTransformOps.td
    "Dialect/Vector/Transforms/VectorTransformsBase.td",   // VectorTransformsBase.td
];

const BASE_URI: &str =
    "https://raw.githubusercontent.com/llvm/llvm-project/refs/heads/main/mlir/include/mlir/";

async fn download_tablegen_files(dest: &Path) -> Result<(), TableGenError> {
    let _ = tokio::fs::create_dir_all(dest).await;

    let client = crate::client::Client::new();

    Ok(for url_path in TABLE_GEN_FILES {
        let url = format!("{BASE_URI}{url_path}");
        let bytes = client
            .get(&url)
            .await
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
        let content = String::from_utf8(bytes.to_vec())
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
        let filename = Path::new(url_path).file_name().unwrap();
        tokio::fs::write(dest.join(filename), content).await?
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;

    static TEST_FIXTURES_DIR: Lazy<Mutex<&Path>> =
        Lazy::new(|| Mutex::new(Path::new("./.fixtures")));

    async fn tablegen_fixtures() -> Vec<String> {
        let fixtures_path = TEST_FIXTURES_DIR.lock().await;

        if !Path::exists(*fixtures_path) {
            tokio::fs::create_dir_all(*fixtures_path).await.unwrap();
            assert!(download_tablegen_files(*fixtures_path).await.is_ok());
        }

        // Read the directory and collect file entries
        let mut dir_entries = tokio::fs::read_dir(*fixtures_path).await.unwrap();
        let mut downloaded_files: Vec<String> = Vec::new();

        while let Some(entry) = dir_entries.next_entry().await.unwrap() {
            let file_path = entry.path();
            // Ensure the file is non-empty
            let metadata = tokio::fs::metadata(&file_path).await.unwrap();
            assert!(metadata.is_file());
            assert!(metadata.len() > 0, "File {:?} is empty", file_path);

            downloaded_files.push(
                String::from_utf8(tokio::fs::read(file_path).await.unwrap().to_vec()).unwrap(),
            );
        }

        // Check that the expected number of files were downloaded
        assert_eq!(
            downloaded_files.len(),
            TABLE_GEN_FILES.len(),
            "Not all files were downloaded"
        );

        downloaded_files
    }

    /// It should be able to download tablegen files to a tmp dir.
    #[cfg(feature = "optional-tests-with-fixtures")]
    #[tokio::test]
    async fn should_download_tablegen_files() {
        let downloaded_files = tablegen_fixtures().await;
        goldie::assert!(downloaded_files.join("\n\n"));
    }

    #[cfg(feature = "optional-tests-with-fixtures")]
    #[tokio::test]
    async fn should_parse_tablegen_files() {
        let downloaded_files = tablegen_fixtures().await;
        assert!(downloaded_files.len() > 0);
    }
}
