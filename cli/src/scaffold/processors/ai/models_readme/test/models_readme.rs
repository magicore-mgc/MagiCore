//! Contract tests for the models readme writer.
//! Test contract cho writer models readme.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path()).unwrap();
    let content = std::fs::read_to_string(target.path().join("models/README.md")).unwrap();
    assert!(content.contains("mgc model pull hf://org/repo"));
}
