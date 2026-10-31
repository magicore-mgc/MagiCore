#[path = "error.rs"]
mod error;
#[path = "rust_build.rs"]
mod implementation;

pub(crate) use implementation::{build_rust_with_env, build_rust_with_managed_source};
#[cfg(test)]
pub(crate) use implementation::{cargo_build_args, managed_source_config};
