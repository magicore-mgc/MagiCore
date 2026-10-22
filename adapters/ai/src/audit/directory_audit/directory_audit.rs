//! Recursively visit discovered model directory entries and audit files.
//! Duyệt đệ quy entry thư mục model và audit file.

use super::super::file_audit::audit_file;
use crate::audit::AuditReport;
use mgc_types::MgResult;
use std::path::Path;

/// Traverse a directory with the existing recursive audit behavior.
/// Duyệt thư mục theo hành vi audit đệ quy hiện tại.
pub(in crate::audit) fn audit_directory(dir: &Path, report: &mut AuditReport) -> MgResult<()> {
    let entries = std::fs::read_dir(dir)?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() {
            audit_file(&path, report)?;
        } else if path.is_dir() {
            audit_directory(&path, report)?;
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
