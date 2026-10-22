//! Verify repeatable serials without treating them as security hashes.
//! Kiểm tra serial tái lập, không xem là hash bảo mật.

use super::super::stable_serial_hash;

#[test]
fn serial_is_stable_for_same_input_and_changes_with_input() {
    assert_eq!(
        stable_serial_hash("report-a"),
        stable_serial_hash("report-a")
    );
    assert_ne!(
        stable_serial_hash("report-a"),
        stable_serial_hash("report-b")
    );
}
