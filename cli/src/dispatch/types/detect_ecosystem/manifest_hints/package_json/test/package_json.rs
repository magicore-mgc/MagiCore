//! Node manifest core-hint extraction contract.

use super::super::declared_core;

#[test]
fn extracts_only_an_explicit_magicore_core() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("package.json"),
        r#"{"name":"fixture","magicore":{"core":"Cloud"}}"#,
    )
    .unwrap();

    assert_eq!(
        declared_core(project.path()).unwrap().as_deref(),
        Some("Cloud")
    );
}

#[test]
fn package_without_magicore_core_has_no_explicit_hint() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("package.json"),
        r#"{"name":"fixture","packageManager":"pnpm@10.0.0"}"#,
    )
    .unwrap();

    assert_eq!(declared_core(project.path()).unwrap(), None);
}

#[test]
fn malformed_json_preserves_the_parser_error_type() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("package.json"), "{not-json").unwrap();

    let error = declared_core(project.path()).unwrap_err();
    assert!(error.downcast_ref::<serde_json::Error>().is_some());
}
