//! Tests the AI scaffold coordinator branch outputs.
//! Test output nhánh của coordinator AI scaffold.

use super::super::AiProcessor;
use tempfile::tempdir;

#[test]
fn mcp_server_key_writes_server_and_server_smoke_test() {
    let root = tempdir().unwrap();
    AiProcessor::files(root.path(), "sample-ai", "mcp-server").unwrap();
    assert!(root.path().join("src/server.py").is_file());
    assert!(root.path().join("tests/test_server.py").is_file());
    assert!(!root.path().join("src/agent.py").exists());
}

#[test]
fn other_keys_keep_the_existing_agent_fallback() {
    let root = tempdir().unwrap();
    AiProcessor::files(root.path(), "sample-ai", "python-agent").unwrap();
    assert!(root.path().join("src/agent.py").is_file());
    assert!(root.path().join("tests/test_agent.py").is_file());
    assert!(!root.path().join("src/server.py").exists());
}
