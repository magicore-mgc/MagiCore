//! Preserve the clean-zero result for an empty operational lock.
//! Giữ kết quả clean-zero khi lock vận hành không có pin.

#[test]
fn empty_pin_report_is_clean() {
    let report = super::report_without_npm_pins(&[]);
    assert!(report.is_clean());
    assert_eq!(report.packages_audited, 0);
}
