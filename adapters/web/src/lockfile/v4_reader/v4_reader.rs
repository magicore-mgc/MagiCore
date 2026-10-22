//! `v4_reader.rs` — v4 document parse and integrity policy verification.
//! parse tài liệu v4 và xác minh policy integrity

use super::super::LockfileError;
use mgc_types::MgResult;
use std::path::Path;

/// v4 document on disk, if the lockfile parses as version 4 — `None`
/// for missing/legacy/unreadable-version locks (legacy reader owns them).
/// (Tài liệu v4 trên đĩa — `None` cho lock thiếu/legacy.)
/// Verify v4 digest and signature math before an adapter consumes or
/// preserves the document. Trust policy remains Warn, matching install.
/// Xác minh digest và chữ ký v4 trước khi adapter dùng hoặc giữ tài liệu;
/// chính sách tin cậy là Warn, đồng nhất với install.
pub(crate) fn verify_v4_document_integrity(
    doc: &mgc_lockfile::canonical::LockfileV4,
) -> MgResult<mgc_lockfile::policy::V4VerifyReport> {
    let report = mgc_lockfile::policy::verify_v4_math(doc).map_err(|error| {
        LockfileError::Integrity(format!("Failed to verify v4 lockfile: {error}"))
    })?;
    mgc_lockfile::policy::enforce_policy(&report, mgc_lockfile::policy::LockPolicyMode::Warn, &[])
        .map_err(|error| {
            LockfileError::Integrity(format!("Failed to verify v4 lockfile: {error}"))
        })?;
    Ok(report)
}

pub(crate) fn read_v4_document_if_present(
    project_root: &Path,
) -> MgResult<Option<mgc_lockfile::canonical::LockfileV4>> {
    let lock_path = project_root.join("mgc.lock");
    let bytes = match mgc_lockfile::read_lockfile_bytes(&lock_path) {
        Ok(bytes) => bytes,
        Err(mgc_lockfile::LockfileError::IoError(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            return Ok(None);
        }
        Err(error) => {
            return Err(LockfileError::Read(format!("Failed to read lockfile: {error}")).into());
        }
    };
    let content = String::from_utf8(bytes).map_err(|error| {
        LockfileError::Encoding(format!("Lockfile is not valid UTF-8: {error}"))
    })?;
    if !mgc_lockfile::detect_lockfile_version(&content).is_ok_and(|version| version == 4) {
        return Ok(None);
    }
    let doc = mgc_lockfile::canonical::parse_v4_document(&content)
        .map_err(|error| LockfileError::Parse(format!("Failed to parse lockfile: {error}")))?;
    verify_v4_document_integrity(&doc)?;
    Ok(Some(doc))
}
