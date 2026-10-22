//! Read a project marker and enforce attestation policy.
//! Đọc marker project và enforce policy attestation.
use std::path::Path;

use super::super::error::Result;
use super::super::identity::read_core_marker_identity;

/// Read core marker from project root.
/// Đọc core marker từ root project.
pub(in crate::project::project::core_marker) fn read_core_marker(
    project_root: &Path,
) -> Result<Option<String>> {
    let Some(core) = read_core_marker_identity(project_root)? else {
        // A missing marker is ordinary for an unclaimed legacy project,
        // but it must not bypass an existing trust anchor after rename/delete.
        // (Marker vắng vẫn hợp lệ với project cũ chưa claim, nhưng không được
        // né anchor sẵn có bằng cách đổi tên/xóa marker.)
        crate::attestation::enforce_missing_marker_attestation(project_root)?;
        return Ok(None);
    };
    // Cryptographic anchor check runs last: plain-text agreement is necessary
    // but not sufficient against deliberate marker edits.
    // (Kiểm tra anchor mật mã chạy cuối: khớp plain-text là cần nhưng chưa đủ.)
    crate::attestation::enforce_live_attestation(project_root, &core)?;
    Ok(Some(core))
}
