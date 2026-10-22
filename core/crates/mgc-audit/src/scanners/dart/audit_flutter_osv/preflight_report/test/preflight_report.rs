//! No-network Dart audit preflight tests.
//! Test tiền kiểm audit Dart không cần mạng.

use super::*;

#[test]
fn empty_inventory_is_clean() {
    let report = report_without_registry_query(&[], &[]).expect("report empty inventory");
    assert_eq!(report.packages_audited, 0);
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(report.scanner_status, ScannerStatus::Available));
}

#[test]
fn skipped_only_inventory_is_partial() {
    let skipped = vec!["local: source 'path' has no advisory mapping".to_string()];
    let report = report_without_registry_query(&[], &skipped).expect("report skipped inventory");
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::Partial {
            scanned: 0,
            skipped: 1,
            ..
        }
    ));
}

#[test]
fn pins_require_registry_lookup() {
    let pins = [OsvPin {
        name: "http".to_string(),
        version: "1.2.2".to_string(),
        ecosystem: "Pub",
    }];
    assert!(report_without_registry_query(&pins, &[]).is_none());
}
