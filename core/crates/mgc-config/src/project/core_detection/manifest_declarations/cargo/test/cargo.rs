//! Cargo core declaration tests.
//! Test khai báo core Cargo.
use super::super::explicit_core;

#[test]
fn package_metadata_declaration_is_read() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname='sample'\nversion='0.1.0'\n[package.metadata.magicore]\ncore='lib'\n",
    )
    .expect("Cargo manifest should be written");

    let detected = explicit_core(project.path()).expect("declaration should parse");

    assert_eq!(detected.as_deref(), Some("lib"));
}
