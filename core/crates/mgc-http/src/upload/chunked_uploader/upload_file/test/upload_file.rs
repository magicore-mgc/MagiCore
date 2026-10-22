// File-upload tests — bounded digest calculation.
// Test upload file — tính digest với bộ nhớ có giới hạn.
#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::full_file_digest;
use std::io::Write;

#[test]
fn full_file_digest_uses_sha256_hex_prefix() {
    let mut file = tempfile::NamedTempFile::new().expect("create temp file");
    file.write_all(b"abc").expect("write input");
    let digest = full_file_digest(file.path()).expect("compute digest");

    assert_eq!(
        digest,
        "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
