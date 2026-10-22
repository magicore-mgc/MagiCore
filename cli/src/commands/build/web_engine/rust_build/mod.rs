#[path = "error.rs"]
mod error;
#[path = "rust_build.rs"]
mod implementation;

pub(crate) use implementation::{build_rust_with_env, build_rust_with_managed_source};
