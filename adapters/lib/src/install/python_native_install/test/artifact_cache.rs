//! Python artifact cache integrity tests.
//! Test tính toàn vẹn cache artifact Python.

use super::support::*;

#[cfg(unix)]
#[test]
fn python_artifact_cache_refuses_symlink_without_reading_target() {
    use sha2::Digest;
    use std::os::unix::fs::symlink;

    let cache = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let bytes = b"outside wheel bytes";
    let expected = hex::encode(sha2::Sha256::digest(bytes));
    let target = outside.path().join("artifact.whl");
    let link = cache.path().join("artifact.whl");
    std::fs::write(&target, bytes).unwrap();
    symlink(&target, &link).unwrap();

    let error = read_cached_python_artifact(&link, &expected).unwrap_err();
    assert!(error.to_string().contains("regular file") || error.to_string().contains("open"));
    assert_eq!(std::fs::read(target).unwrap(), bytes);
}

#[test]
fn python_artifact_cache_reuses_only_digest_verified_regular_files() {
    use sha2::Digest;

    let cache = tempfile::tempdir().unwrap();
    let bytes = b"wheel bytes";
    let expected = hex::encode(sha2::Sha256::digest(bytes));
    let artifact = cache.path().join("artifact.whl");

    assert_eq!(
        read_cached_python_artifact(&artifact, &expected).unwrap(),
        None
    );
    std::fs::write(&artifact, bytes).unwrap();
    assert_eq!(
        read_cached_python_artifact(&artifact, &expected).unwrap(),
        Some(bytes.to_vec())
    );
    std::fs::write(&artifact, b"tampered wheel").unwrap();
    let error = read_cached_python_artifact(&artifact, &expected).unwrap_err();
    assert!(error.to_string().contains("integrity mismatch"));
}
