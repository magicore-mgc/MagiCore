//! Tests for SARIF driver trust metadata.
//! Kiểm thử metadata trust của SARIF driver.

#![allow(clippy::unwrap_used)]

use super::*;
use mgc_types::adapter::{AuditReport, ScannerStatus};

#[test]
fn available_status_uses_the_trusted_driver_name() {
    let report = AuditReport::clean(3);
    let driver = build_driver(&report);

    assert_eq!(driver.name, "mgc-audit");
    assert_eq!(
        driver.information_uri,
        "https://github.com/mingd-153/MagiCore"
    );
    assert_eq!(driver.version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn every_non_available_status_uses_the_unverified_driver_name() {
    let partial = AuditReport {
        packages_audited: 1,
        vulnerability_count: 0,
        vulnerabilities: Vec::new(),
        scanner_status: ScannerStatus::Partial {
            scanned: 1,
            skipped: 1,
            reasons: vec!["lockfile unavailable".to_string()],
        },
    };
    let reports = [
        partial,
        AuditReport::tool_missing("osv", "install scanner"),
        AuditReport::unsupported_ecosystem("unknown"),
        AuditReport::scanner_failed("osv", "request failed"),
    ];
    for report in &reports {
        assert_eq!(
            build_driver(report).name,
            "mgc-audit (UNVERIFIED — scanner did not complete)"
        );
    }
}
