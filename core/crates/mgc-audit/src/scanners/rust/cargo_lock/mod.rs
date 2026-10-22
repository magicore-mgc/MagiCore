//! Cargo.lock parsing exports and local error boundary.
//! Export parser Cargo.lock và ranh giới lỗi cục bộ.

mod cargo_lock;
mod document;
mod error;
mod package;
mod source_policy;

pub(super) use cargo_lock::read;
pub(super) use error::CargoLockError;

use crate::scanners::osv::OsvPin;
use mgc_types::{MgError, MgResult};

/// Read crates.io pins while preserving this parser's public error messages.
/// Đọc pin crates.io và giữ nguyên nội dung lỗi public của parser này.
pub fn read_cargo_lock_pins(raw: &str) -> MgResult<(Vec<OsvPin>, Vec<String>)> {
    read(raw).map_err(|error| MgError::Other(error.to_string()))
}

#[cfg(test)]
#[path = "test/cargo_lock.rs"]
mod tests;
