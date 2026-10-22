//! Contract tests for the project manifest writer.
//! Test contract cho writer project manifest.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path(), "sample-ai", "python-agent").unwrap();
    let content = std::fs::read_to_string(target.path().join("pyproject.toml")).unwrap();
    assert!(content.contains("name = \"sample-ai\""));
    assert!(content.contains("framework = \"python-agent\""));
}
