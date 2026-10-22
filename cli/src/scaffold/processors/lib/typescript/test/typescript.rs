use crate::scaffold::processors::lib::LibProcessor;

#[test]
fn typescript_scaffold_emits_publishable_dist() {
    let temp = tempfile::tempdir().expect("tempdir");
    LibProcessor::files(temp.path(), "demo-lib", "typescript").expect("ts scaffold");
    let config = std::fs::read_to_string(temp.path().join("tsconfig.json")).expect("config");
    assert!(config.contains("\"outDir\": \"dist\""));
    assert!(config.contains("\"declaration\": true"));
    assert!(config.contains("\"strict\": true"));
}
