//! Script policy typed error and source-chain tests.
//! Test lỗi typed và source chain của policy script.

use super::super::{ScriptsPolicyError, load_scripts_table};
use std::error::Error;

#[test]
fn missing_file_and_missing_table_remain_no_policy() {
    let directory = tempfile::tempdir().expect("temporary project directory should be created");
    assert!(
        load_scripts_table(directory.path())
            .expect("missing project config should not be an error")
            .is_none()
    );

    std::fs::write(directory.path().join("mgc.toml"), "name = 'sample'\n")
        .expect("minimal project config should be written");
    assert!(
        load_scripts_table(directory.path())
            .expect("missing scripts table should not be an error")
            .is_none()
    );
}

#[test]
fn malformed_toml_has_typed_error_with_source_and_compatible_display() {
    let directory = tempfile::tempdir().expect("temporary project directory should be created");
    std::fs::write(directory.path().join("mgc.toml"), "[[[invalid")
        .expect("malformed project config should be written");

    let error = load_scripts_table(directory.path()).expect_err("malformed TOML must fail closed");
    assert!(matches!(&error, ScriptsPolicyError::InvalidToml { .. }));
    assert!(error.source().is_some());
    assert!(error.to_string().starts_with("invalid TOML in "));
}

#[test]
fn invalid_policy_shape_has_typed_error_with_source() {
    let directory = tempfile::tempdir().expect("temporary project directory should be created");
    std::fs::write(directory.path().join("mgc.toml"), "[scripts]\nallow = 42\n")
        .expect("invalid policy table should be written");

    let error = load_scripts_table(directory.path()).expect_err("invalid policy schema must fail");
    assert!(matches!(
        &error,
        ScriptsPolicyError::DecodeScriptsTable { .. }
    ));
    assert!(error.source().is_some());
    assert!(error.to_string().starts_with("invalid [scripts] table in "));
}
