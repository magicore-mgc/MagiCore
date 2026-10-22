//! Python artifact-cache integrity tests.
//! Test toàn vẹn cache artifact Python.

use sha2::Digest;

#[cfg(unix)]
#[test]
fn rejects_symlinks_before_returning_cached_bytes() {
    use std::os::unix::fs::symlink;

    let cache = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let bytes = b"outside wheel bytes";
    let expected = hex::encode(sha2::Sha256::digest(bytes));
    let target = outside.path().join("artifact.whl");
    let link = cache.path().join("artifact.whl");
    std::fs::write(&target, bytes).unwrap();
    symlink(&target, &link).unwrap();

    let error = crate::install::read_cached_python_artifact(&link, &expected).unwrap_err();
    assert!(matches!(&error, mgc_types::MgError::Integrity(_)));
    assert!(error.to_string().contains("regular file") || error.to_string().contains("open"));
    assert_eq!(std::fs::read(target).unwrap(), bytes);
}

#[test]
fn returns_only_regular_entries_with_the_expected_digest() {
    let cache = tempfile::tempdir().unwrap();
    let bytes = b"wheel bytes";
    let expected = hex::encode(sha2::Sha256::digest(bytes));
    let artifact = cache.path().join("artifact.whl");

    let error = crate::install::read_cached_python_artifact(cache.path(), &expected).unwrap_err();
    assert!(matches!(&error, mgc_types::MgError::Integrity(_)));
    assert!(error.to_string().contains("not a regular file"));

    assert_eq!(
        crate::install::read_cached_python_artifact(&artifact, &expected).unwrap(),
        None
    );
    std::fs::write(&artifact, bytes).unwrap();
    assert_eq!(
        crate::install::read_cached_python_artifact(&artifact, &expected).unwrap(),
        Some(bytes.to_vec())
    );
    std::fs::write(&artifact, b"tampered wheel").unwrap();
    let error = crate::install::read_cached_python_artifact(&artifact, &expected).unwrap_err();
    assert!(matches!(&error, mgc_types::MgError::Integrity(_)));
    assert!(error.to_string().contains("integrity mismatch"));
}
