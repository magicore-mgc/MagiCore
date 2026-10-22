//! Cargo.lock parser error exports and local tests.
//! Export lỗi parser Cargo.lock và test cục bộ.

mod error;
pub(in crate::scanners::rust) use error::CargoLockError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
