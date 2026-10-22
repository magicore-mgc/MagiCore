//! Tests streamed hashing and symlink rejection.
//! Kiểm thử hash theo luồng và từ chối symlink.

use crate::cas::store::ContentStore;
use crate::cas::store::StoreError;

#[test]
fn hashes_regular_file_contents() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("payload.bin");
    let bytes = vec![0x5a; 96 * 1024];
    std::fs::write(&input, &bytes).unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();

    let actual = store.verify(&input).unwrap();

    assert_eq!(actual.as_hex(), blake3::hash(&bytes).to_hex().to_string());
}

#[cfg(unix)]
#[test]
fn rejects_a_symlink_before_hashing_its_target() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target.bin");
    let link = temp.path().join("link.bin");
    std::fs::write(&target, b"target").unwrap();
    symlink(&target, &link).unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();

    let error = store.verify(&link).unwrap_err();

    assert!(
        matches!(error, StoreError::Io { msg, .. } if msg == "path is a symlink, refusing to verify")
    );
}

#[test]
fn rejects_a_directory_as_a_non_regular_file() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("directory");
    std::fs::create_dir(&input).unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();

    let error = store.verify(&input).unwrap_err();

    assert!(matches!(error, StoreError::Io { msg, .. } if msg == "path is not a regular file"));
}
