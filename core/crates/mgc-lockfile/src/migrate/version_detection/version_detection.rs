//! Detects the declared lockfile schema version.
//! Nhận diện phiên bản schema được khai báo trong lockfile.

use crate::{LockfileError, LockfileResult};

/// Detect lockfile version from TOML string — Phát hiện version lockfile từ chuỗi TOML
pub fn detect_lockfile_version(toml_str: &str) -> LockfileResult<u8> {
    // Try to parse the declared version — Thử parse version được khai báo.
    let parsed: toml::Value = toml::from_str(toml_str)?;

    let version = parsed
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| LockfileError::ParseError("missing version field".to_string()))?;

    match version {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        version if crate::v4::is_v4_schema_version(version) => Ok(4),
        _ => Err(LockfileError::ParseError(format!(
            "unknown version: {}",
            version
        ))),
    }
}
