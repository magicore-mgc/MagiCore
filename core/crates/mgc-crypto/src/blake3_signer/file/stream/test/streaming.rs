//! Tests bounded-memory file hashing and I/O errors.
//! Kiểm tra hash file với bộ nhớ giới hạn và lỗi I/O.
//! Kiểm tra hash file bộ nhớ giới hạn và lỗi I/O.

use crate::blake3_signer::Blake3Hasher;
use std::io::Write;

#[test]
fn streams_empty_and_nonempty_files() {
    let empty = tempfile::NamedTempFile::new().expect("test operation must succeed");
    assert_eq!(
        Blake3Hasher::hash_file(empty.path()).expect("test operation must succeed"),
        Blake3Hasher::hash_bytes(b"")
    );

    let mut file = tempfile::NamedTempFile::new().expect("test operation must succeed");
    let content = vec![0x5au8; 32 * 1024];
    file.write_all(&content)
        .expect("test operation must succeed");
    assert_eq!(
        Blake3Hasher::hash_file(file.path()).expect("test operation must succeed"),
        Blake3Hasher::hash_bytes(&content)
    );
}

#[test]
fn returns_io_error_for_missing_file() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("missing-input");
    assert!(Blake3Hasher::hash_file(&path).is_err());
}
