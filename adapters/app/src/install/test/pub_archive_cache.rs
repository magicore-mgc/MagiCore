//! Archive cache integrity regression tests.
//! Kiểm tra checksum, giới hạn và symlink của cache Pub.
#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn verified_cache_roundtrips_and_tamper_fails() {
    let root = tempfile::tempdir().unwrap();
    let bytes = b"verified archive fixture";
    let digest = hex::encode(Sha256::digest(bytes));
    assert!(load_verified(root.path(), &digest).unwrap().is_none());
    assert!(!root.path().join(ARCHIVE_DIRECTORY).exists());
    save_verified(root.path(), &digest, bytes).unwrap();
    assert_eq!(load_verified(root.path(), &digest).unwrap().unwrap(), bytes);
    std::fs::write(archive_path(root.path(), &digest).unwrap(), b"tampered").unwrap();
    assert!(load_verified(root.path(), &digest).is_err());
}

#[test]
fn invalid_digest_and_wrong_bytes_do_not_publish_cache() {
    let root = tempfile::tempdir().unwrap();
    assert!(save_verified(root.path(), "../escape", b"payload").is_err());
    assert!(save_verified(root.path(), &"0".repeat(64), b"payload").is_err());
    assert!(!root.path().join(ARCHIVE_DIRECTORY).exists());
}

#[test]
fn bounded_cache_read_rejects_oversized_archive() {
    let root = tempfile::tempdir().unwrap();
    let bytes = b"archive";
    let digest = hex::encode(Sha256::digest(bytes));
    save_verified(root.path(), &digest, bytes).unwrap();
    assert!(load_with_limit(root.path(), &digest, 3).is_err());
}

#[cfg(unix)]
#[test]
fn linked_archive_and_cache_directory_are_rejected() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let digest = hex::encode(Sha256::digest(b"archive"));
    std::os::unix::fs::symlink(outside.path(), root.path().join(ARCHIVE_DIRECTORY)).unwrap();
    assert!(load_verified(root.path(), &digest).is_err());
    assert!(save_verified(root.path(), &digest, b"archive").is_err());
    std::fs::remove_file(root.path().join(ARCHIVE_DIRECTORY)).unwrap();
    std::fs::create_dir(root.path().join(ARCHIVE_DIRECTORY)).unwrap();
    let payload = outside.path().join("payload");
    std::fs::write(&payload, b"archive").unwrap();
    std::os::unix::fs::symlink(&payload, archive_path(root.path(), &digest).unwrap()).unwrap();
    assert!(load_verified(root.path(), &digest).is_err());
}
