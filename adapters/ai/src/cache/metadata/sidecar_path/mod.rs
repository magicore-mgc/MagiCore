//! Metadata sidecar-path exports — khai báo path helper dùng chung.

mod sidecar_path;

pub(in crate::cache) use sidecar_path::metadata_path;

#[cfg(test)]
#[path = "test/sidecar_path.rs"]
mod tests;
