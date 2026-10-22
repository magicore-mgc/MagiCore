//! Report-fold owner cases — test trường hợp của owner gom report.
use super::collect_reports;
use mgc_types::adapter::{AuditReport, ScannerStatus};

#[test]
fn available_reports_sum_packages_and_scans() {
    let reports = vec![
        ("node:osv".to_owned(), AuditReport::clean(3)),
        ("rust:cargo-audit".to_owned(), AuditReport::clean(4)),
    ];

    let summary = collect_reports(&reports);

    assert_eq!(summary.packages_audited, 7);
    assert_eq!(summary.scanned, 2);
    assert!(summary.any_available);
}

#[test]
fn partial_report_keeps_counts_and_reasons() {
    let report = AuditReport {
        packages_audited: 5,
        vulnerability_count: 0,
        vulnerabilities: vec![],
        scanner_status: ScannerStatus::Partial {
            scanned: 5,
            skipped: 1,
            reasons: vec!["direct-only coverage".to_owned()],
        },
    };
    let reports = vec![("go:fallback".to_owned(), report)];

    let summary = collect_reports(&reports);

    assert_eq!(summary.packages_audited, 5);
    assert_eq!(summary.scanned, 5);
    assert_eq!(summary.skipped, 1);
    assert_eq!(summary.reasons, vec!["direct-only coverage".to_owned()]);
    assert!(!summary.all_unsupported);
}
