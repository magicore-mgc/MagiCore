//! Parse the TOML document and read its package collection.
//! Parse document TOML và lấy danh sách package.

use toml::Value;

use super::super::CargoLockError;

/// Parse raw Cargo.lock TOML.
/// Parse TOML thô của Cargo.lock.
pub(in crate::scanners::rust::cargo_lock) fn parse(raw: &str) -> Result<Value, CargoLockError> {
    toml::from_str(raw).map_err(CargoLockError::InvalidToml)
}

/// Return the package array, or `None` when the document has no package key.
/// Trả package array, hoặc `None` nếu document không có key package.
pub(in crate::scanners::rust::cargo_lock) fn package_records(
    document: &Value,
) -> Result<Option<&Vec<Value>>, CargoLockError> {
    match document.get("package") {
        Some(value) => value
            .as_array()
            .map(Some)
            .ok_or(CargoLockError::PackageFieldNotArray),
        None => Ok(None),
    }
}
