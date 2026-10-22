//! Reports for inputs that do not require a registry request.
//! Tạo report cho input không cần truy vấn registry.

use crate::scanners::osv::OsvPin;
use mgc_types::adapter::{AuditReport, ScannerStatus};

pub(in crate::scanners::dart::audit_flutter_osv) fn report_without_registry_query(
    pins: &[OsvPin],
    skipped: &[String],
) -> Option<AuditReport> {
    if !pins.is_empty() {
        return None;
    }
    if skipped.is_empty() {
        return Some(AuditReport::clean(0));
    }
    Some(AuditReport {
        packages_audited: 0,
        vulnerability_count: 0,
        vulnerabilities: vec![],
        scanner_status: ScannerStatus::Partial {
            scanned: 0,
            skipped: skipped.len(),
            reasons: skipped.to_vec(),
        },
    })
}

#[cfg(test)]
#[path = "test/preflight_report.rs"]
mod tests;
