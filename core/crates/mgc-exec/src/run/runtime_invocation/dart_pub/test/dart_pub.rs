// Focused test for case-insensitive pub identification.
// Test tập trung cho nhận diện pub không phân biệt hoa thường.

use super::*;
use std::ffi::OsString;

#[test]
fn matches_pub_after_runtime_flags() {
    let argv = vec![
        OsString::from("dart"),
        OsString::from("--disable-dart-dev"),
        OsString::from("PUB"),
        OsString::from("get"),
    ];
    assert_eq!(dart_pub_argument(&argv), Some((2, "PUB".to_string())));
}
