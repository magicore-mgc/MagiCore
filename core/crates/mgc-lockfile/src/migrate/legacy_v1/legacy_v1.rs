//! Owns the legacy v1 lockfile DTOs and strict TOML parser.
//! Sở hữu DTO lockfile v1 cũ và parser TOML nghiêm ngặt.

use crate::{LockfileError, LockfileResult};
use serde::{Deserialize, Serialize};

/// Lockfile v1 structure (legacy) — Cấu trúc lockfile v1 (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockfileV1 {
    pub version: String,
    #[serde(rename = "package")]
    pub packages: Vec<PackageV1>,
}

/// Package v1 structure (no integrity field) — Cấu trúc package v1 (không có integrity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageV1 {
    pub name: String,
    pub version: String,
    pub resolved: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
}
/// Parse lockfile v1 from TOML string — Parse lockfile v1 từ chuỗi TOML
pub fn parse_lockfile_v1(toml_str: &str) -> LockfileResult<LockfileV1> {
    let lockfile_v1: LockfileV1 = toml::from_str(toml_str)?;

    if lockfile_v1.version != "1" {
        return Err(LockfileError::ParseError(format!(
            "expected version 1, got {}",
            lockfile_v1.version
        )));
    }

    Ok(lockfile_v1)
}
