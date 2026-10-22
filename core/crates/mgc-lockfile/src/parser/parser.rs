//! Parses legacy and v4 TOML while preserving schema-specific identity.
//! Parse TOML legacy và v4, đồng thời giữ identity riêng theo schema.

use crate::{LOCKFILE_SCHEMA_VERSION, Lockfile, LockfileResult};

use super::error::DocumentParseError;

/// Parsed lock document without discarding schema-specific identity.
/// Tài liệu lock đã parse, giữ nguyên identity theo schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockDocument {
    Legacy(Lockfile),
    V4(crate::canonical::LockfileV4),
}

/// Parse a legacy lockfile from TOML text.
/// Parse lockfile legacy từ nội dung TOML.
pub fn parse_lockfile(toml_str: &str) -> LockfileResult<Lockfile> {
    let value: toml::Value = toml::from_str(toml_str).map_err(DocumentParseError::Toml)?;
    parse_legacy_value(value).map_err(Into::into)
}

/// Deserialize and validate one legacy schema from a parsed TOML value.
/// Deserialize và xác thực một schema legacy từ TOML value đã parse.
fn parse_legacy_value(value: toml::Value) -> Result<Lockfile, DocumentParseError> {
    let lockfile: Lockfile = value.try_into().map_err(DocumentParseError::Toml)?;
    if lockfile.version != "2" && lockfile.version != LOCKFILE_SCHEMA_VERSION {
        return Err(DocumentParseError::UnsupportedVersion(lockfile.version));
    }
    Ok(lockfile)
}

/// Dispatch TOML lockfiles by their declared schema version.
/// Chọn parser theo version khai báo trong lockfile.
pub fn parse_document(toml_str: &str) -> LockfileResult<LockDocument> {
    parse_document_inner(toml_str).map_err(Into::into)
}

fn parse_document_inner(toml_str: &str) -> Result<LockDocument, DocumentParseError> {
    let value: toml::Value = toml::from_str(toml_str).map_err(DocumentParseError::Toml)?;
    let version = value
        .get("version")
        .and_then(toml::Value::as_str)
        .ok_or(DocumentParseError::MissingVersion)?
        .to_string();

    if crate::v4::is_v4_schema_version(&version) {
        let lock: crate::canonical::LockfileV4 =
            value.try_into().map_err(DocumentParseError::Toml)?;
        crate::canonical::validate_v4_revision(&lock).map_err(DocumentParseError::V4Revision)?;
        return Ok(LockDocument::V4(lock));
    }
    match version.as_str() {
        "2" | LOCKFILE_SCHEMA_VERSION => parse_legacy_value(value).map(LockDocument::Legacy),
        other => Err(DocumentParseError::UnsupportedVersion(other.to_string())),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
