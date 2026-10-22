//! Contract tests for the model runtime writer.
//! Test contract cho writer model runtime.

use tempfile::tempdir;

#[test]
fn writes_its_owned_output() {
    let target = tempdir().unwrap();
    super::super::write(target.path()).unwrap();
    let content = std::fs::read_to_string(target.path().join("src/mgc_model_runtime.py")).unwrap();
    assert!(content.contains("class ModelRuntimeConfig"));
    assert!(content.contains("self.target_bits = target_bits"));
}
