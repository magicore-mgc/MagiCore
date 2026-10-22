//! Required Cargo.lock package fields and source extraction.
//! Field package bắt buộc và trích xuất source từ Cargo.lock.

mod package;
pub(super) use package::{name_version, source};

#[cfg(test)]
#[path = "test/package.rs"]
mod tests;
