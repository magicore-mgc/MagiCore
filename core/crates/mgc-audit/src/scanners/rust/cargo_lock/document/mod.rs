//! Cargo.lock TOML document parsing and top-level package-array shape.
//! Parse document TOML Cargo.lock và hình dạng package array cấp cao nhất.

mod document;
pub(super) use document::{package_records, parse};

#[cfg(test)]
#[path = "test/document.rs"]
mod tests;
