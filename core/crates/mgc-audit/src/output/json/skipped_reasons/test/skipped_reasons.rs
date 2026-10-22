//! Tests for JSON scanner coverage reason projection.
//! Kiểm thử ánh xạ lý do coverage scanner cho JSON.

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
fn all_states_keep_existing_skip_reason_behavior() {
    let cases = [
        (ScannerStatus::Available, Vec::new()),
        (
            ScannerStatus::Partial {
                scanned: 1,
                skipped: 2,
                reasons: vec!["first reason".to_string(), "second reason".to_string()],
            },
            vec!["first reason".to_string(), "second reason".to_string()],
        ),
        (
            ScannerStatus::ToolMissing {
                tool: "osv".to_string(),
                remediation: "install scanner".to_string(),
            },
            vec!["osv: install scanner".to_string()],
        ),
        (
            ScannerStatus::UnsupportedEcosystem {
                ecosystem: "unknown".to_string(),
            },
            vec!["unknown".to_string()],
        ),
        (
            ScannerStatus::Failed {
                scanner: "osv".to_string(),
                reason: "request failed".to_string(),
            },
            vec!["osv: request failed".to_string()],
        ),
    ];

    for (status, expected) in cases {
        assert_eq!(skipped_reasons(&report(status)), expected);
    }
}
