//! Truthy-value vocabulary tests.
//! Test tập giá trị truthy.

use super::super::is_truthy_value;

#[test]
fn accepts_only_the_existing_truthy_values() {
    for value in ["1", " true ", "YES", "On"] {
        assert!(is_truthy_value(value), "expected truthy value: {value}");
    }

    for value in ["", "0", "false", "no", "off", "enabled", " true-ish "] {
        assert!(!is_truthy_value(value), "expected false value: {value}");
    }
}
