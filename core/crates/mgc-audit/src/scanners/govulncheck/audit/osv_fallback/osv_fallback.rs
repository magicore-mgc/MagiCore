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
    // Prefer the locked graph: `mgc.lock` holds every resolved module, so an
    // advisory against a transitive dependency is visible. `go.mod` holds direct
    // requires only, and reading it made every Go audit report Partial even
    // though MagiCore had already resolved the whole graph.
    // (Ưu tiên graph đã lock: `mgc.lock` có mọi module đã resolve nên advisory
    // cho dependency bắc cầu thấy được. `go.mod` chỉ có require trực tiếp.)
    if let Some(locked) = crate::scanners::govulncheck::lock::go_pins_from_mgc_lock(project_root)? {
        if locked.is_empty() {
            return Ok(AuditReport {
                packages_audited: 0,
                vulnerability_count: 0,
                vulnerabilities: Vec::new(),
                scanner_status: ScannerStatus::Partial {
                    scanned: 0,
                    skipped: 1,
                    reasons: vec![
                        "mgc.lock pins no Go modules; run `mgc install` to resolve the graph"
                            .to_string(),
                    ],
                },
            });
        }
        let mut report = audit_osv_pins(&locked).await?;
        // Full-graph coverage from the lock is Available unless the query failed.
        // (Phủ toàn graph từ lock là Available trừ khi truy vấn lỗi.)
        if matches!(report.scanner_status, ScannerStatus::Partial { .. }) {
            report.scanner_status = ScannerStatus::Available;
        }
        return Ok(report);
    }

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
