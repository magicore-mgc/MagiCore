// Focused tests for compact inline-code options.
// Test tập trung cho dạng option code inline liền nhau.

use super::*;
use std::ffi::OsString;

#[test]
fn compact_inline_code_is_not_scanned_as_a_module() {
    let argv = vec![
        OsString::from("python"),
        OsString::from("-cimport pip"),
    ];
    assert_eq!(python_script_argument(&argv), None);
}
