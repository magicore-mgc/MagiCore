//! Audit Report nearest-owner contract tests.
//! Test contract gần owner audit report.

use crate::adapter::*;
use crate::package::PackageId;

/// A scan that inspected nothing is not clean.
/// (Scan không kiểm gì thì không sạch.)
#[test]
fn audit_report_of_zero_packages_is_not_clean() {
    let report = AuditReport::clean(0);
    assert!(!report.is_clean());
    assert!(matches!(
        report.scanner_status,
        crate::adapter::ScannerStatus::Partial { .. }
    ));
}

#[test]
fn audit_report_is_clean_when_a_package_was_inspected() {
    let report = AuditReport::clean(1);
    assert!(report.is_clean());
}

#[test]
fn audit_report_clean_tracks_packages_audited_without_vulns() {
    let report = AuditReport::clean(3);
    assert_eq!(report.packages_audited, 3);
    assert_eq!(report.vulnerability_count, 0);
    assert!(report.is_clean());
}

#[test]
fn audit_report_not_clean_with_vulnerabilities_vec() {
    let report = AuditReport {
        packages_audited: 1,
        vulnerability_count: 1,
        vulnerabilities: vec![Vulnerability {
            package: PackageId::parse("test@1.0.0").unwrap(),
            title: "vuln".to_string(),
            severity: "high".to_string(),
            cve: "CVE-123".to_string(),
            severity_level: VulnerabilitySeverity::High,
            patched_versions: None,
            url: None,
            scanner: None,
            ecosystem: None,
            evidence_at: None,
            finding_class: FindingClass::Vulnerability,
        }],
        scanner_status: ScannerStatus::Available,
    };
    assert!(!report.is_clean());
}

#[test]
fn audit_report_not_clean_with_nonzero_count() {
    let report = AuditReport {
        packages_audited: 3,
        vulnerability_count: 3,
        vulnerabilities: vec![],
        scanner_status: ScannerStatus::Available,
    };
    assert!(!report.is_clean());
}

/// The five-state contract (Tech Lead P0-1): is_clean() must be false for
/// EVERY non-Available state — a report from a scanner that never ran is
/// never "clean", even with zero findings.
/// Hợp đồng 5 trạng thái: is_clean() phải sai với MỌI trạng thái khác
/// Available — report từ scanner chưa chạy không bao giờ "sạch", dù 0 finding.
#[test]
fn audit_report_never_clean_when_scanner_not_available() {
    let partial = AuditReport {
        packages_audited: 3,
        vulnerability_count: 0,
        vulnerabilities: vec![],
        scanner_status: ScannerStatus::Partial {
            scanned: 3,
            skipped: 2,
            reasons: vec!["two deps unscreened".to_string()],
        },
    };
    assert!(!partial.is_clean());

    let tool_missing = AuditReport::tool_missing("cargo-audit", "cargo install cargo-audit");
    assert!(!tool_missing.is_clean());

    let unsupported = AuditReport::unsupported_ecosystem("flutter");
    assert!(!unsupported.is_clean());

    let failed = AuditReport::scanner_failed("pip-audit", "schema drift");
    assert!(!failed.is_clean());

    // Available + zero findings + at least one inspected package is the ONLY
    // clean state.
    // (Available + 0 finding + ít nhất một package đã kiểm là trạng thái sạch
    // DUY NHẤT.)
    assert!(AuditReport::clean(1).is_clean());
    assert!(!AuditReport::clean(0).is_clean());
}
