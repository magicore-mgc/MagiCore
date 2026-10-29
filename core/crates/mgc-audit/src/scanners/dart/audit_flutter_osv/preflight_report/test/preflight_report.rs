//! No-network Dart audit preflight tests.
//! Test tiền kiểm audit Dart không cần mạng.

use super::*;

/// An empty inventory is NOT clean: no package was inspected.
///
/// This test used to require `Available`, which is how a Dart project with no
/// resolved packages produced a green audit.
/// (Inventory rỗng KHÔNG sạch: không package nào được kiểm. Test này từng đòi
/// `Available`, chính là cách project Dart chưa resolve ra audit xanh.)
#[test]
fn empty_inventory_is_not_clean() {
    let report = report_without_registry_query(&[], &[]).expect("report empty inventory");
    assert_eq!(report.packages_audited, 0);
    assert_eq!(report.vulnerability_count, 0);
    assert!(
        matches!(report.scanner_status, ScannerStatus::Partial { .. }),
        "an empty inventory must report Partial, got {:?}",
        report.scanner_status
    );
    assert!(!report.is_clean());
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
