//! OSV report skipped-source projection tests.
//! Test ánh xạ nguồn bị bỏ qua lên report OSV.

use super::*;

#[test]
fn skipped_sources_set_partial_counts_and_reasons() {
    let skipped = vec!["local: source 'git' has no advisory mapping".to_string()];
    let report = apply_skipped_packages(AuditReport::clean(3), 3, &skipped);
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::Partial {
            scanned: 3,
            skipped: 1,
            ..
        }
    ));
    assert_eq!(report.packages_audited, 3);
}

#[test]
fn no_skipped_sources_preserve_osv_status() {
    let report = apply_skipped_packages(AuditReport::clean(2), 2, &[]);
    assert!(matches!(report.scanner_status, ScannerStatus::Available));
}
