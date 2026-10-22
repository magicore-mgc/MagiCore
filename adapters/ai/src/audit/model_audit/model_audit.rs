//! Audit one model path and coordinate file or directory scanning.
//! Audit một đường dẫn model và điều phối quét file hoặc thư mục.

use super::super::AuditReport;
use super::super::directory_audit::audit_directory;
use super::super::file_audit::audit_file;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Audit a model directory or a single model file.
/// Audit thư mục model hoặc một file model.
pub async fn audit_model(path: &Path) -> MgResult<AuditReport> {
    if !path.exists() {
        return Err(MgError::Other(format!(
            "Model path not found: {}",
            path.display()
        )));
    }

    let mut report = AuditReport::new(&path.to_string_lossy());

    if path.is_file() {
        audit_file(path, &mut report)?;
    } else if path.is_dir() {
        audit_directory(path, &mut report)?;
    }

    Ok(report)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
