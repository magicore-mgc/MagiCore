//! Coordinate bounded Cargo.lock input and the shared OSV audit.
//! Điều phối đọc Cargo.lock có giới hạn và audit OSV dùng chung.

use crate::scanners::osv::audit_osv_pins;
use mgc_types::MgResult;
use mgc_types::adapter::{AuditReport, ScannerStatus};
use std::path::Path;

/// Audit locked crates.io dependencies through MGC's OSV client.
/// Audit dependency crates.io đã khóa qua OSV client của MGC.
pub async fn audit_rust(project_root: &Path) -> MgResult<AuditReport> {
    let lock_path = project_root.join("Cargo.lock");
    let raw = crate::scanners::read_bounded_project_input(&lock_path, "Cargo.lock")?;
    let (pins, skipped) = super::super::read_cargo_lock_pins(&raw)?;
    let mut report = audit_osv_pins(&pins).await?;
    if !skipped.is_empty() {
        report.scanner_status = ScannerStatus::Partial {
            scanned: report.packages_audited,
            skipped: skipped.len(),
            reasons: skipped,
        };
    }
    Ok(report)
}
