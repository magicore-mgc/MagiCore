use crate::scaffold::processors::lib::LibProcessor;

#[test]
fn rust_scaffold_is_standalone_inside_parent_workspace() {
    let temp = tempfile::tempdir().expect("tempdir");
    LibProcessor::files(temp.path(), "demo-lib", "rust").expect("rust scaffold");
    let manifest = std::fs::read_to_string(temp.path().join("Cargo.toml")).expect("manifest");
    assert!(manifest.contains("edition = \"2024\""));
    assert!(manifest.contains("\n[workspace]\n"));
}
