//! pyproject.toml core declaration tests.
//! Test khai báo core pyproject.toml.
use super::super::explicit_core;

#[test]
fn tool_metadata_declaration_is_read() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[tool.magicore]\ncore='ai'\n",
    )
    .expect("Python manifest should be written");

    let detected = explicit_core(project.path()).expect("declaration should parse");

    assert_eq!(detected.as_deref(), Some("ai"));
}
