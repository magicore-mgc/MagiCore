//! Read Python project metadata into the shared manifest model.
//! Đọc metadata dự án Python vào mô hình manifest dùng chung.

mod error;
mod parse;
pub use parse::parse_pyproject_manifest;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
