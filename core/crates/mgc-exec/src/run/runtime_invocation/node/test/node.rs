// Focused tests for opaque inline source and unknown-option handling.
// Test tập trung cho source inline opaque và option chưa biết.

use super::*;
use std::ffi::OsString;

#[test]
fn does_not_treat_inline_source_or_unknown_option_values_as_scripts() {
    let inline = vec![
        OsString::from("node"),
        OsString::from("--eval"),
        OsString::from("require('npm')"),
    ];
    assert_eq!(node_script_argument(&inline), None);

    let unknown = vec![
        OsString::from("node"),
        OsString::from("--future-flag"),
        OsString::from("/project/npm-cli.js"),
    ];
    assert_eq!(node_script_argument(&unknown), None);
}
