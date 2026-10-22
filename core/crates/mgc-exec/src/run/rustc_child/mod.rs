//! Rust compiler-child facade — export constrained argv recognition.
//! Facade compiler child Rust — export nhận diện argv có giới hạn.

#[path = "rustc_child.rs"]
mod implementation;
pub(super) use implementation::{
    is_rustc_compilation_argv, is_rustc_toolchain_child, is_rustc_toolchain_wrapper_argv,
    is_rustc_wrapper_argv, is_rustdoc_test_compilation_argv,
};
