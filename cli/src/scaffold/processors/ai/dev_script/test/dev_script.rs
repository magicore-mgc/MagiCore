//! Contract tests for the dev script writer.
//! Test contract cho writer dev script.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path(), "src/agent.py").unwrap();
    let content = std::fs::read_to_string(target.path().join("scripts/run_dev.sh")).unwrap();
    assert!(content.contains("python3 src/agent.py"));
    assert!(content.contains(".mgc-optimizer/ai_runtime.env"));
}
