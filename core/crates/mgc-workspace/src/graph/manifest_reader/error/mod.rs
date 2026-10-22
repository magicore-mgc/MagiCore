//! Typed local manifest-reader errors.
//! Lỗi có kiểu cục bộ của bộ đọc manifest.

mod error;

pub(super) use error::ManifestReadError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
