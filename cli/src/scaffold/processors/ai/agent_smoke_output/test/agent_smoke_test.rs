//! Contract tests for the agent smoke test writer.
//! Test contract cho writer agent smoke test.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path()).unwrap();
    let content = std::fs::read_to_string(target.path().join("tests/test_agent.py")).unwrap();
    assert!(content.contains("def test_agent_runs"));
    assert!(content.contains("def test_runtime_config_inert"));
}
