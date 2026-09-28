# mlir-testutils

Utilities for writing tests in crates that use [melior](https://github.com/mlir-rs/melior).

For the utilities that assert equality between two things you can enable the `similar-asserts` feature.
Enabling this feature makes the assertions use [similar-asserts](https://crates.io/crates/similar-asserts)
instead of the standard [`assert_eq`](https://doc.rust-lang.org/std/macro.assert_eq.html) macro.

## Build requirements

Building this crate requires an LLVM/MLIR 20 installation. The transitive `mlir-sys`
and `tblgen` build scripts need its `llvm-config` and TableGen tools, so set both
variables to that installation's prefix before running Cargo:

```sh
# macOS/Linux with Homebrew
brew install llvm@20
export MLIR_SYS_200_PREFIX="$(brew --prefix llvm@20)"
export TABLEGEN_200_PREFIX="$MLIR_SYS_200_PREFIX"
```

For a non-Homebrew installation, replace the value with the path to the LLVM
installation (the directory containing `bin/llvm-config`). These settings resolve
build errors such as `failed to find correct version (20.x.x) of llvm-config`
from `mlir-sys` or `tblgen`.
