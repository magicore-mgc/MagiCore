//! Build the honest CocoaPods unsupported-advisory report.
//! Tạo report trung thực khi advisory CocoaPods chưa được hỗ trợ.

use crate::scanners::osv::OsvPin;
use mgc_types::adapter::{AuditReport, ScannerStatus};

/// Preserve inventory counts and parser skip reasons while reporting no advisory coverage.
/// Giữ số lượng kiểm kê và lý do parse bị skip khi advisory chưa có coverage.
pub(in crate::scanners::osv) fn report_cocoapods_inventory(
    pins: Vec<OsvPin>,
    skipped: Vec<String>,
) -> AuditReport {
    let mut reasons = vec![format!(
        "cocoapods: {} pods pinned in Podfile.lock — OSV.dev carries no CocoaPods advisory ecosystem (verified: 'pods' rejected by the live API)",
        pins.len()
    )];
    reasons.extend(skipped);
    AuditReport {
        packages_audited: pins.len(),
        vulnerability_count: 0,
        vulnerabilities: vec![],
        scanner_status: ScannerStatus::UnsupportedEcosystem {
            ecosystem: format!(
                "objc/cocoapods ({} pods; {} unparsable entries)",
                pins.len(),
                reasons.len() - 1
            ),
        },
    }
}
