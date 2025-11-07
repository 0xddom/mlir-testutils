# mlir-testutils

Utilities for writing tests in crates that use [melior](https://github.com/mlir-rs/melior).

For the utilities that assert equality between two things you can enable the `similar-asserts` feature.
Enabling this feature makes the assertions use [similar-asserts](https://crates.io/crates/similar-asserts) 
instead of the standard [`assert_eq`](https://doc.rust-lang.org/std/macro.assert_eq.html) macro. 
