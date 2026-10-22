//! Manifest source-order contract.

use super::super::read;

#[test]
fn reads_node_then_cargo_then_python_core_declarations() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("package.json"),
        r#"{"name":"fixture","magicore":{"core":"web"}}"#,
    )
    .unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\n[package.metadata.magicore]\ncore = \"game\"\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[tool.magicore]\ncore = \"ai\"\n",
    )
    .unwrap();

    assert_eq!(
        read(project.path()).unwrap(),
        vec!["web".to_owned(), "game".to_owned(), "ai".to_owned()]
    );
}
