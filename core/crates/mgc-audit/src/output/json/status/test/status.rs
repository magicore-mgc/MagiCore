//! Tests for stable JSON scanner status slugs.
//! Kiểm thử slug trạng thái scanner JSON ổn định.

use super::*;
use mgc_types::adapter::{AuditReport, ScannerStatus};

fn report(status: ScannerStatus) -> AuditReport {
    AuditReport {
        packages_audited: 0,
        vulnerability_count: 0,
        vulnerabilities: Vec::new(),
        scanner_status: status,
    }
}

#[test]
fn every_scanner_state_keeps_its_existing_slug() {
    let cases = [
        (ScannerStatus::Available, "available"),
        (
            ScannerStatus::Partial {
                scanned: 0,
                skipped: 1,
                reasons: vec!["unknown".to_string()],
            },
            "partial",
        ),
        (
            ScannerStatus::ToolMissing {
                tool: "osv".to_string(),
                remediation: "install scanner".to_string(),
            },
            "tool_missing",
        ),
        (
            ScannerStatus::UnsupportedEcosystem {
                ecosystem: "unknown".to_string(),
            },
            "unsupported_ecosystem",
        ),
        (
            ScannerStatus::Failed {
                scanner: "osv".to_string(),
                reason: "request failed".to_string(),
            },
            "failed",
        ),
    ];

    for (status, expected) in cases {
        assert_eq!(status_slug(&report(status)), expected);
    }
}
