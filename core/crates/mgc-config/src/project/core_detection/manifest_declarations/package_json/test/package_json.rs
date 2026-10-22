//! package.json declaration behavior tests.
//! Test hành vi khai báo package.json.
use super::super::explicit_core;

#[test]
fn declaration_is_canonicalized() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("package.json"),
        r#"{"magicore":{"core":" Cloud "}}"#,
    )
    .expect("package manifest should be written");

    let detected = explicit_core(project.path()).expect("declaration should parse");

    assert_eq!(detected.as_deref(), Some("clo"));
}

#[test]
fn malformed_manifest_keeps_its_source_error() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(project.path().join("package.json"), "{")
        .expect("package manifest should be written");

    let error = explicit_core(project.path()).expect_err("invalid JSON should fail");

    assert!(error.to_string().contains("EOF"));
    assert!(std::error::Error::source(&error).is_some());
}
