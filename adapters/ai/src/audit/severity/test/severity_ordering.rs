//! Severity levels keep their documented ordering.
//! Severity phải giữ đúng thứ tự ưu tiên.

use super::*;

#[test]
fn test_severity_ordering() {
    assert!(Severity::Critical > Severity::High);
    assert!(Severity::High > Severity::Medium);
    assert!(Severity::Medium > Severity::Low);
}
