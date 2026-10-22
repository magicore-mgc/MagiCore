//! Query direct Go require pins through the shared OSV client.
//! (Truy vấn pin require Go trực tiếp qua OSV client dùng chung.)

use crate::scanners::govulncheck::read_go_mod_requires;
use crate::scanners::osv::audit_osv_pins;
use crate::scanners::read_bounded_project_input;
use mgc_types::MgResult;
use mgc_types::adapter::{AuditReport, ScannerStatus};
use std::path::Path;

pub(in crate::scanners::govulncheck::audit) async fn audit_go_osv_fallback(
    project_root: &Path,
) -> MgResult<AuditReport> {
    let raw = read_bounded_project_input(&project_root.join("go.mod"), "go.mod")?;
    let (pins, mut skipped) = read_go_mod_requires(&raw);
    if pins.is_empty() {
        skipped.push(
            "go.mod contains no versioned require pins; transitive coverage is not established"
                .to_string(),
        );
        return Ok(AuditReport {
            packages_audited: 0,
            vulnerability_count: 0,
            vulnerabilities: Vec::new(),
            scanner_status: ScannerStatus::Partial {
                scanned: 0,
                skipped: skipped.len(),
                reasons: skipped,
            },
        });
    }
    skipped.push(
        "OSV fallback covers direct require pins only — go.sum transitives need govulncheck for symbol-level precision"
            .to_string(),
    );
    let mut report = audit_osv_pins(&pins).await?;
    // Findings are real, but coverage is direct-only: Partial, never a
    // full Available. A failed OSV query stays Failed (fail-closed).
    // Finding thật nhưng phủ direct-only: Partial, không Available tròn.
    if matches!(report.scanner_status, ScannerStatus::Available) {
        report.scanner_status = ScannerStatus::Partial {
            scanned: pins.len(),
            skipped: skipped.len(),
            reasons: skipped,
        };
    }
    Ok(report)
}
