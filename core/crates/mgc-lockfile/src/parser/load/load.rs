//! Loads bounded lockfile bytes and applies schema compatibility gates.
//! Nạp byte lockfile có giới hạn và áp dụng cổng tương thích schema.

use std::path::Path;

use crate::{Lockfile, LockfileError, LockfileResult};

use super::super::parser::LockDocument;
use super::error::LockLoadError;

/// Load and parse a lock document without flattening v4 package identity.
/// Nạp và parse lock document mà không làm phẳng identity package v4.
pub fn load_lock_document(path: &Path) -> LockfileResult<LockDocument> {
    let bytes = super::super::bounded_reader::read_lockfile_bytes(path)?;
    let content = std::str::from_utf8(&bytes)
        .map_err(|error| LockfileError::ParseError(format!("invalid UTF-8: {error}")))?;
    super::super::parser::parse_document(content)
}

/// Require the legacy model without silently flattening a v4 document.
/// Yêu cầu model legacy, không âm thầm làm phẳng document v4.
pub(in crate::parser) fn require_legacy_document(
    document: LockDocument,
) -> LockfileResult<Lockfile> {
    match document {
        LockDocument::Legacy(lockfile) => Ok(lockfile),
        LockDocument::V4(_) => Err(LockLoadError::LossyLegacyModel.into()),
    }
}

/// Load through the legacy API; callers needing v4 must use `LockDocument`.
/// Nạp qua API legacy; caller cần v4 phải dùng `LockDocument`.
pub fn load_lockfile(path: &Path) -> LockfileResult<Lockfile> {
    require_legacy_document(load_lock_document(path)?)
}

/// Fail closed with guidance when a v3 workflow encounters v4 on disk.
/// Fail-closed kèm hướng dẫn khi workflow v3 gặp lock v4 trên đĩa.
pub fn ensure_v3_readable_lock(path: &Path) -> LockfileResult<()> {
    let bytes = match super::super::bounded_reader::read_lockfile_bytes(path) {
        Ok(bytes) => bytes,
        Err(LockfileError::IoError(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let text = std::str::from_utf8(&bytes).map_err(|error| {
        LockfileError::ParseError(format!("lockfile is not valid UTF-8: {error}"))
    })?;
    if crate::migrate::detect_lockfile_version(text).is_ok_and(|version| version == 4) {
        return Err(LockLoadError::V4UnderV3 {
            path: path.display().to_string(),
        }
        .into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
