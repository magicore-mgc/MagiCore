use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

use super::super::{
    find_pylock_file, find_requirements_file, read_requirements_pins, read_uv_lock_pins,
};
use crate::scanners::{osv, python_pins_from_mgc_lock, read_bounded_project_input};

/// Audit Python pins with MGC's parser and OSV client; no package-manager spawn.
/// Dùng parser và OSV client của MGC để audit Python; không spawn package manager.
pub async fn audit_python(project_root: &Path) -> MgResult<AuditReport> {
    if let Some(owned) = python_pins_from_mgc_lock(project_root)? {
        return osv::audit_osv_pins(&owned).await;
    }
    let uv_lock = project_root.join("uv.lock");
    let (pins, mut skipped, requirements_only) = if uv_lock.is_file() {
        let raw = read_bounded_project_input(&uv_lock, "uv.lock")?;
        let (pins, skipped) = read_uv_lock_pins(&raw)?;
        (pins, skipped, false)
    } else if find_requirements_file(project_root).is_some() {
        let (pins, skipped) = read_requirements_pins(project_root)?;
        (pins, skipped, true)
    } else {
        let reason = if project_root.join("pyproject.toml").is_file() {
            "pyproject.toml has no mgc.lock or supported resolved uv.lock; audit is unverified"
        } else if find_pylock_file(project_root).is_some() {
            "PEP 751 pylock is not yet supported by the native Python audit parser"
        } else {
            "no mgc.lock, uv.lock, or requirements*.txt found for Python audit"
        };
        return Ok(AuditReport::scanner_failed("mgc-python-osv", reason));
    };
    let mut report = osv::audit_osv_pins(&pins).await?;
    if requirements_only || !skipped.is_empty() {
        if skipped.is_empty() {
            skipped.push("Python dependency coverage is incomplete".to_string());
        }
        report.scanner_status = mgc_types::adapter::ScannerStatus::Partial {
            scanned: report.packages_audited,
            skipped: skipped.len(),
            reasons: skipped,
        };
    }
    Ok(report)
}
