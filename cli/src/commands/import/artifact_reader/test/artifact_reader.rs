//! Artifact reader safety cases — ca an toàn reader artifact.
#![allow(clippy::unwrap_used)]

use super::super::{read_optional_lock, read_optional_signature};
#[test]
fn import_artifact_reader_distinguishes_missing_from_invalid_file_types() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("missing.lock");
    assert_eq!(read_optional_lock(&missing).unwrap(), None);

    let regular = dir.path().join("mgc.lock");
    std::fs::write(&regular, b"lock-bytes").unwrap();
    assert_eq!(
        read_optional_lock(&regular).unwrap(),
        Some(b"lock-bytes".to_vec())
    );

    let directory = dir.path().join("signature-directory");
    std::fs::create_dir(&directory).unwrap();
    assert!(read_optional_signature(&directory).is_err());
}

#[cfg(unix)]
#[test]
fn import_artifact_reader_refuses_symlinks() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("target");
    let link = dir.path().join("mgc.lock");
    std::fs::write(&target, b"must-not-follow").unwrap();
    symlink(&target, &link).unwrap();
    assert!(read_optional_lock(&link).is_err());
    assert_eq!(std::fs::read(target).unwrap(), b"must-not-follow");
}
