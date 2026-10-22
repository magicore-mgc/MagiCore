//! Queries Maven pins and projects skipped entries into report status.
//! Truy vấn pin Maven và đưa entry bị bỏ qua vào trạng thái report.

use crate::scanners::osv::{OsvPin, audit_osv_pins};
use mgc_types::MgResult;
use mgc_types::adapter::{AuditReport, ScannerStatus};

/// Query OSV for pins with the shared skipped→Partial tail.
/// Query OSV cho ghim, kèm đuôi skipped→Partial chung.
pub(in crate::scanners::maven) async fn query_maven_pins(
    pins: &(Vec<OsvPin>, Vec<String>),
) -> MgResult<AuditReport> {
    let (pins, skipped) = pins;
    if pins.is_empty() && skipped.is_empty() {
        return Ok(AuditReport::clean(0));
    }
    if pins.is_empty() {
        return Ok(AuditReport {
            packages_audited: 0,
            vulnerability_count: 0,
            vulnerabilities: vec![],
            scanner_status: ScannerStatus::Partial {
                scanned: 0,
                skipped: skipped.len(),
                reasons: skipped.clone(),
            },
        });
    }
    let mut report = audit_osv_pins(pins).await?;
    if !skipped.is_empty() {
        report.scanner_status = ScannerStatus::Partial {
            scanned: pins.len(),
            skipped: skipped.len(),
            reasons: skipped.clone(),
        };
    }
    Ok(report)
}

#[cfg(test)]
#[path = "test/query_maven_pins.rs"]
mod tests;
