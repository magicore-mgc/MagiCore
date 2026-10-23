//! Contract tests for the mcp smoke test writer.
//! Test contract cho writer mcp smoke test.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path()).unwrap();
    let content = std::fs::read_to_string(target.path().join("tests/test_server.py")).unwrap();
    assert!(content.contains("def test_server_module_imports"));
}
