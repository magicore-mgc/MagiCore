//! Extract the required identity and optional source from a lock record.
//! Trích xuất định danh bắt buộc và source tùy chọn từ record lock.

use toml::Value;

use super::super::CargoLockError;

/// Return package name/version with the original validation messages.
/// Trả name/version với thông báo xác thực cũ.
pub(in crate::scanners::rust::cargo_lock) fn name_version(
    package: &Value,
) -> Result<(&str, &str), CargoLockError> {
    let name = package
        .get("name")
        .and_then(Value::as_str)
        .ok_or(CargoLockError::PackageNameInvalid)?;
    let version = package
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| CargoLockError::PackageVersionInvalid {
            package_name: name.to_string(),
        })?;
    Ok((name, version))
}

/// Read a string source, treating missing/non-string values like the old parser.
/// Đọc source dạng chuỗi; giá trị thiếu/sai kiểu giữ hành vi parser cũ.
pub(in crate::scanners::rust::cargo_lock) fn source(package: &Value) -> Option<&str> {
    package.get("source").and_then(Value::as_str)
}
