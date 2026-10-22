//! Aggregate Swift pin results without hiding skipped dependencies.
//! Tổng hợp kết quả pin Swift và giữ rõ các dependency bị bỏ qua.

use crate::scanners::osv::{OsvPin, audit_osv_pins};
use mgc_types::MgResult;
use mgc_types::adapter::{AuditReport, ScannerStatus};

/// Keep empty, partial, and fully queried Swift outcomes distinct.
/// Giữ riêng trạng thái rỗng, một phần và đã truy vấn đầy đủ của Swift.
pub(in crate::scanners::osv) async fn report_swift_pins(
    pins: Vec<OsvPin>,
    skipped: Vec<String>,
) -> MgResult<AuditReport> {
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
                reasons: skipped,
            },
        });
    }
    let mut report = audit_osv_pins(&pins).await?;
    if !skipped.is_empty() {
        report.scanner_status = ScannerStatus::Partial {
            scanned: pins.len(),
            skipped: skipped.len(),
            reasons: skipped,
        };
    }
    Ok(report)
}
