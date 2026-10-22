//! Project selected pins and query results into the audit report contract.
//! Chuyển pin được chọn và query result thành hợp đồng audit report.

use mgc_types::Vulnerability;
use mgc_types::adapter::{AuditReport, ScannerStatus};

/// Report clean-zero or honest partial coverage when no npm pin is scannable.
/// Báo clean-zero hoặc độ phủ Partial trung thực khi không có pin npm scan được.
pub(in crate::audit) fn report_without_npm_pins(jsr_pins: &[String]) -> AuditReport {
    if jsr_pins.is_empty() {
        return AuditReport::clean(0);
    }

    AuditReport {
        packages_audited: 0,
        vulnerability_count: 0,
        vulnerabilities: vec![],
        scanner_status: ScannerStatus::Partial {
            scanned: 0,
            skipped: jsr_pins.len(),
            reasons: jsr_partial_reasons(jsr_pins),
        },
    }
}

/// Report the completed npm scan and retain JSR pins as skipped Partial coverage.
/// Báo scan npm đã chạy và giữ pin JSR ở trạng thái coverage Partial.
pub(in crate::audit) fn report_after_npm_scan(
    npm_pins: &[(String, String)],
    jsr_pins: &[String],
    vulnerabilities: Vec<Vulnerability>,
) -> AuditReport {
    let vulnerability_count = vulnerabilities.len();
    let scanner_status = if jsr_pins.is_empty() {
        ScannerStatus::Available
    } else {
        ScannerStatus::Partial {
            scanned: npm_pins.len(),
            skipped: jsr_pins.len(),
            reasons: jsr_partial_reasons(jsr_pins),
        }
    };

    AuditReport {
        packages_audited: npm_pins.len(),
        vulnerability_count,
        vulnerabilities,
        scanner_status,
    }
}

fn jsr_partial_reasons(jsr_pins: &[String]) -> Vec<String> {
    jsr_pins
        .iter()
        .map(|package| format!("jsr package '{package}' has no npm advisory mapping yet"))
        .collect()
}
