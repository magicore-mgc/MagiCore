use crate::scaffold::processors::lib::LibProcessor;

/// `create-lib go` must scaffold Go files (not the rust fallback) —
/// go.mod with a module line so detect_language + native add work.
#[test]
fn go_scaffold_emits_go_mod_not_cargo() {
    let temp = tempfile::tempdir().expect("tempdir");
    LibProcessor::files(temp.path(), "demo-lib", "go").expect("go scaffold");
    assert!(
        !temp.path().join("Cargo.toml").exists(),
        "go scaffold must not contain Cargo.toml"
    );
    let gomod = std::fs::read_to_string(temp.path().join("go.mod")).expect("go.mod exists");
    assert!(gomod.contains("module demo-lib"), "module line: {gomod}");
}
