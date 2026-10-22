//! Safely update MGC-owned Python manifest fields.
//! Cập nhật an toàn các trường manifest Python thuộc MGC.

mod error;
mod write;
pub use write::write_pyproject_manifest;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
