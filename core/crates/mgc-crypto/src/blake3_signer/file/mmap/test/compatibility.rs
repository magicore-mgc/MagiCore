//! Ensures the former mmap method matches safe streaming.
//! Đảm bảo method mmap cũ cho kết quả giống streaming an toàn.
//! Đảm bảo method mmap cũ khớp cách streaming an toàn.

use crate::blake3_signer::Blake3Hasher;
use std::io::Write;

#[test]
fn legacy_mmap_method_matches_streaming_digest() {
    let mut file = tempfile::NamedTempFile::new().expect("test operation must succeed");
    let content = b"compatibility digest";
    file.write_all(content)
        .expect("test operation must succeed");
    assert_eq!(
        Blake3Hasher::hash_file_mmap(file.path()).expect("test operation must succeed"),
        Blake3Hasher::hash_file(file.path()).expect("test operation must succeed")
    );
}
