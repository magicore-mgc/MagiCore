//! Keep JSR-only and mixed npm/JSR coverage explicitly Partial.
//! Giữ coverage chỉ JSR và trộn npm/JSR ở trạng thái Partial tường minh.

use mgc_types::adapter::ScannerStatus;

#[test]
fn jsr_only_and_mixed_coverage_are_partial() {
    let jsr = vec!["jsr:@scope/pkg".to_string()];
    let jsr_only = super::report_without_npm_pins(&jsr);
    assert!(matches!(
        jsr_only.scanner_status,
        ScannerStatus::Partial { scanned: 0, skipped: 1, ref reasons }
            if reasons == &["jsr package 'jsr:@scope/pkg' has no npm advisory mapping yet"]
    ));

    let mixed = super::report_after_npm_scan(
        &[("leftpad".to_string(), "1.0.0".to_string())],
        &jsr,
        Vec::new(),
    );
    assert!(matches!(
        mixed.scanner_status,
        ScannerStatus::Partial { scanned: 1, skipped: 1, ref reasons }
            if reasons == &["jsr package 'jsr:@scope/pkg' has no npm advisory mapping yet"]
    ));
    assert_eq!(mixed.packages_audited, 1);
}
