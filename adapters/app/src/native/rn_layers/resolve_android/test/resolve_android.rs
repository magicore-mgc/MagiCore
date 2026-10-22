#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify Gradle versions stay exact Maven ranges.
//! Xác nhận version Gradle thành range Maven chính xác.

use crate::native::rn_layers::resolve_android::resolve_android::exact_maven_version_range;

#[test]
fn gradle_pin_is_encoded_as_an_exact_maven_range() {
    assert_eq!(exact_maven_version_range("1.2.3"), "[1.2.3]");
}
