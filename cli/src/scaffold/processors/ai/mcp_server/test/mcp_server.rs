//! Contract tests for the mcp server writer.
//! Test contract cho writer mcp server.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path(), "sample_ai").unwrap();
    let content = std::fs::read_to_string(target.path().join("src/server.py")).unwrap();
    assert!(content.contains("MagiCore MCP Server (sample_ai) initialized!"));
}
