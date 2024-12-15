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
//! ## fljúga handahófi nvgpu dialect support
//!
//! Implements nvgpu mlir high level dialect support.
//! nvgpu abstracts nvvm dialect with Device-specific PTX Asm instructions.
//! **NOTE:** translation is not perfect, **fljúga handahófi** may provide some hacks and low-level optimizations in the future, or directly contribute to mlir.
