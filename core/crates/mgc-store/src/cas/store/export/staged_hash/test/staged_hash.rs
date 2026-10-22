//! Staged Hash source regression test.
//! Test hồi quy source cho owner staged hash.

use std::fs;

#[test]
fn hashes_file_contents_with_blake3() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("staged");
    fs::write(&path, b"hash me").unwrap();

    let actual = super::super::staged_hash::hash_file(&path).unwrap();

    assert_eq!(actual, blake3::hash(b"hash me").to_hex().to_string());
}
