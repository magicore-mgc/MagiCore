//! Preserve OSV findings while marking skipped package sources partial.
//! Giữ finding OSV và đánh dấu nguồn package bị bỏ qua là partial.

use mgc_types::adapter::{AuditReport, ScannerStatus};

pub(in crate::scanners::dart::audit_flutter_osv) fn apply_skipped_packages(
    mut report: AuditReport,
    scanned: usize,
    skipped: &[String],
) -> AuditReport {
    if !skipped.is_empty() {
        report.scanner_status = ScannerStatus::Partial {
            scanned,
            skipped: skipped.len(),
            reasons: skipped.to_vec(),
        };
    }
    report
}

#[cfg(test)]
#[path = "test/skipped_projection.rs"]
mod tests;
