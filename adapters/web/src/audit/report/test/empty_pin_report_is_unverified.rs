//! An empty operational lock inspects nothing, so it cannot be clean.
//! Lock vận hành rỗng không kiểm gì nên không thể sạch.

#[test]
fn empty_pin_report_is_unverified() {
    let report = super::report_without_npm_pins(&[]);

    // The previous name asserted the opposite. Reporting `Available` for zero
    // pins meant `mgc audit` printed "✔ No vulnerabilities" on a project whose
    // graph was never inspected — the fail-open this module now refuses.
    // (Tên cũ khẳng định ngược lại. Báo `Available` cho 0 pin khiến `mgc audit`
    // in "✔ No vulnerabilities" cho project chưa từng được kiểm.)
    assert!(!report.is_clean());
    assert_eq!(report.packages_audited, 0);
    assert!(matches!(
        report.scanner_status,
        mgc_types::adapter::ScannerStatus::Partial { .. }
    ));
}
