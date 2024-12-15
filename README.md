# fljúga handahófi

*fljúga handahófi* is a *rust_codegen_mlir* implementation.

## STATUS: WIP

### Supported Internal Dialects

 - [ ] [affine](https://mlir.llvm.org/docs/Dialects/Affine/) transforms
 - [ ] [arith](https://mlir.llvm.org/docs/Dialects/ArithOps/) transforms
 - [ ] [async](https://mlir.llvm.org/docs/Dialects/AsyncDialect/) runtimes support 
 - [ ] [bufferization](https://mlir.llvm.org/docs/Dialects/BufferizationOps/) to accumulate certain shapes for more effective hardware acceleration 
 - [ ] [cf](https://mlir.llvm.org/docs/Dialects/ControlFlowDialect/) basic unstructured program control flow constructs (if/when applicable)
 - [ ] [complex](https://mlir.llvm.org/docs/Dialects/ComplexOps/) numbers operations
 - [ ] [dlti](https://mlir.llvm.org/docs/Dialects/DLTIDialect/) data layout for specific compilation targets handling 
 - [ ] [func](https://mlir.llvm.org/docs/Dialects/Func/) calls and respective operations
 - [ ] [gpu](https://mlir.llvm.org/docs/Dialects/GPU/)  abstracts gpgpu computations, currently applicable primarily to
       [nvgpu](https://mlir.llvm.org/docs/Dialects/NVGPU/)
 - [ ] [index](https://mlir.llvm.org/docs/Dialects/IndexOps/) pointer indexing operations
 - [ ] [irdl](https://mlir.llvm.org/docs/Dialects/IRDL/) SSA-based IR generation
 - [ ] [linalg](https://mlir.llvm.org/docs/Dialects/Linalg/) linear algebra support 
 - [ ] [llvm](https://mlir.llvm.org/docs/Dialects/LLVM/) dialect backports the current llvm codegen to mlir
 - [ ] [math](https://mlir.llvm.org/docs/Dialects/MathOps/) complex math operations (trig and polynomials)
 - [ ] [memref](https://mlir.llvm.org/docs/Dialects/MemRef/) memory allocation operations
 - [ ] [mesh](https://mlir.llvm.org/docs/Dialects/Mesh/) sharding operations over cluster of devices
 - [ ] [ML program](https://mlir.llvm.org/docs/Dialects/MLProgramOps/) written in modern ML frameworks 
       ([TensorFlow](https://www.tensorflow.org/)/[JAX](https://jax.readthedocs.io/en/latest/index.html))
 - [ ] [pdl](https://mlir.llvm.org/docs/Dialects/PDLOps/) pattern definition language for MLIR rewrites
 - [ ] [pdl interp](https://mlir.llvm.org/docs/Dialects/PDLInterpOps/) low-level PDL interpreter operations
 - [ ] [polynomial](https://mlir.llvm.org/docs/Dialects/PolynomialDialect/) single variable polynomials
 - [ ] [ptr](https://mlir.llvm.org/docs/Dialects/PtrOps/) pointer primitive
 - [ ] [quant](https://mlir.llvm.org/docs/Dialects/QuantDialect/) quantization operations
 - [ ] [scf](https://mlir.llvm.org/docs/Dialects/SCFDialect/) structured control flow (`if` conditions and `for` loops)
 - [ ] [shape](https://mlir.llvm.org/docs/Dialects/ShapeDialect/) represents abastract structures
 - [ ] [tensor](https://mlir.llvm.org/docs/Dialects/TensorOps/) operations
 - [ ] [tensor operator set](https://mlir.llvm.org/docs/Dialects/TOSA/) generalized tensor operations, hardware-agnostic over CPU/GPU/NPU
 - [ ] [sparse tensor](https://mlir.llvm.org/docs/Dialects/SparseTensorOps/) operations
 - [ ] [ub](https://mlir.llvm.org/docs/Dialects/UBOps/) marks undefined behavior constant poisoning

### Supported External Dialects

 - [ ] [OpenACC](https://openacc.org/) (**[CRATE](./crates/dialect-openacc)**) fortran/C heterogeneous CPU/GPU accelerated operations
 - [ ] [AMD GPU](https://mlir.llvm.org/docs/Dialects/AMDGPU/) (**[CRATE](./crates/dialect-amdgpu)**) dialect for [ROCm](https://www.amd.com/en/products/software/rocm.html),
       translates to [rocdl](https://mlir.llvm.org/docs/Dialects/ROCDLDialect/)
 - [ ] [Intel AMX](https://www.intel.com/content/www/us/en/products/docs/accelerator-engines/what-is-intel-amx.html) (**[CRATE](./crates/dialect-intel-amx)**)
   fancy and luxury ML simd for expensive xeon CPU's  
 - [ ] [ARM NEON](https://developer.arm.com/Architectures/Neon) (**[CRATE](./crates/dialect-arm-neon)**) basic ARM SIMD dialect
 - [ ] [ARM SVE](https://developer.arm.com/Architectures/Scalable%20Vector%20Extensions) (**[CRATE](./crates/dialect-arm-sve)**) fancy ARM SIMD dialect for modern ARMv9+ ISA (as of 2023)
 - [ ] [ARM SME](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/arm-scalable-matrix-extension-introduction) (**[CRATE](./crates/dialect-arm-sme)**) floating point operations
 - [ ] [emitc](https://mlir.llvm.org/docs/Dialects/EmitC/) (**[CRATE](./crates/dialect-emit-c)**) emits C, can be used for debugging
 - [ ] [mpi](https://mlir.llvm.org/docs/Dialects/MPI/) (**[CRATE](./crates/dialect-mpi)**) abstracts proprietary MPI4.0 communication interfaces 
 - [ ] [nvgpu](https://mlir.llvm.org/docs/Dialects/NVGPU/) (**[CRATE](./crates/dialect-nvgpu)**) high-level nvidia gpu dialect
 - [ ] [nvvm](https://mlir.llvm.org/docs/Dialects/NVVMDialect/) (**[CRATE](./crates/dialect-nvvm)**) low-level nvidia ptx vm dialect, compiled from nvgpu 
 - [ ] [omp](https://mlir.llvm.org/docs/Dialects/OpenMPDialect/) (**[CRATE](./crates/omp)**) low-level [OpenMP](https://www.openmp.org/) dialect
 - [ ] [rocdl](https://mlir.llvm.org/docs/Dialects/ROCDLDialect/) (**[CRATE](./crates/dialect-rocdl)**) low-level AMD gpu dialect
 - [ ] [vcix](https://mlir.llvm.org/docs/Dialects/VCIXDialect/) (**[CRATE](./crates/dialect-riscv-vcix)**) RISC-V [SiFive VCIX](https://www.sifive.com/technology/vectors)
 - [ ] [x86 vector](https://mlir.llvm.org/docs/Dialects/X86Vector/) (**[CRATE](./crates/dialect-x86-vector)**) for the most common SSE/AVX x86 SIMD instruction sets
 - [ ] [xegpu](https://mlir.llvm.org/docs/Dialects/XeGPU/) (**[CRATE](./crates/dialect-intel-xe)**) for Intel XE graphics and related accelerators
 - [ ] [spir-v](https://mlir.llvm.org/docs/Dialects/SPIR-V/) (**[CRATE](./crates/dialect-spirv)**) for direct low-level gpgpu support, superceeds other gpu dialects, with various outcomes 

### Internal crates

 - [ ] [mlir-codegen](./crates/mlir-codegen) macro generates respective dialect bindings from parsed tablegen files
 - [ ] [tablegen](./crates/tablegen) files parser in [winnow](https://github.com/winnow-rs/winnow)
 - [ ] [læra að fljúga](./crates/laera-fljuga) is a machine learning framework for *fljúga handahófi*, provides [ML program](https://mlir.llvm.org/docs/Dialects/MLProgramOps/) bindings
## Development

Install [asdf](https://asdf-vm.com/) first.

```bash
asdf plugin add rust
asdf install
asdf current # should list all the correct versions from .tools-versions file

# wasm-pack for rust, as well 
cargo install cargo-run-bin

# fljúga uses feature flags
rustup default nightly
rustup component add clippy-preview
rustup component add rustfmt

cargo bin --install

# extend PATH for cargo, if none was previously available
echo "export PATH=\"\$PATH:\$HOME/.cargo/bin\"" >> ~/.zshrc # or .bashrc

# load rust env for the current rust version specified in .tools-versions file
echo ". \"\$HOME/.asdf/installs/rust/$(cat .tool-versions| grep rust | awk '{print $2}')/env\"" >> ~/.zshrc # or .bashrc

# Linting
cargo bin licensure -i **/*.rs
rustfmt --edition 2024 **/*.rs

```

## 