#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn slot_hashes_the_canonical_project_path() {
    let identities = tempfile::tempdir().expect("identities directory");
    let project = tempfile::tempdir().expect("project directory");
    let canonical = project.path().canonicalize().expect("canonical path");
    let expected = identities.path().join(crate::attestation::blake3_hex(
        canonical.to_string_lossy().as_bytes(),
    ));
    assert_eq!(attestation_dir(identities.path(), project.path()), expected);
}

#[test]
fn reads_exact_project_config_bytes() {
    let project = tempfile::tempdir().expect("project directory");
    let bytes = b"name = \"demo\"\necosystem = \"web\"\n";
    std::fs::write(project.path().join("mgc.toml"), bytes).expect("write config");
    assert_eq!(
        read_config_bytes(project.path()).expect("read config"),
        bytes
    );
}
