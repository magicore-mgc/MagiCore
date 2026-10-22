//! Contract tests for the model configuration writer.
//! Test contract cho writer model configuration.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path(), "sample-ai").unwrap();
    let content = std::fs::read_to_string(target.path().join("configs/model_config.json")).unwrap();
    assert!(content.contains("\"model_name\": \"sample-ai\""));
}
