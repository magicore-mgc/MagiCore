use crate::scaffold::processors::lib::LibProcessor;

/// `create-lib java` must scaffold a pom.xml (not the rust fallback).
#[test]
fn java_scaffold_emits_pom_not_cargo() {
    let temp = tempfile::tempdir().expect("tempdir");
    LibProcessor::files(temp.path(), "demo-lib", "java").expect("java scaffold");
    assert!(
        !temp.path().join("Cargo.toml").exists(),
        "java scaffold must not contain Cargo.toml"
    );
    let pom = std::fs::read_to_string(temp.path().join("pom.xml")).expect("pom exists");
    assert!(
        pom.contains("<artifactId>demo-lib</artifactId>"),
        "coords: {pom}"
    );
}
