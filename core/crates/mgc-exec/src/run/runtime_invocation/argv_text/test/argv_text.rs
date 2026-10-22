// Focused tests for absent and valid argv positions.
// Test tập trung cho vị trí argv thiếu và hợp lệ.

use super::*;
use std::ffi::OsString;

#[test]
fn reads_only_existing_utf8_values() {
    let argv = vec![OsString::from("node"), OsString::from("script.js")];
    assert_eq!(argv_text(&argv, 0), Some("node"));
    assert_eq!(argv_text(&argv, 2), None);
    assert_eq!(argv_text(&[], 0), None);
}
