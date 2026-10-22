//! Read and authenticate cached Python package artifacts.
//! Đọc và xác minh artifact package Python đã cache.

mod artifact_cache;
mod error;

pub(super) use artifact_cache::read_cached_python_artifact;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
