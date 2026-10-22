//! Metafile-input error formatting tests.
//! Test format lỗi input metafile.

use super::super::MetafileInputsError;

#[test]
fn rejected_input_diagnostic_does_not_include_an_input_path() {
    let message = MetafileInputsError::OutsideAllowedRoots.to_string();

    assert!(message.contains("outside installed dependency roots"));
    assert!(!message.contains("/Users/"));
}
