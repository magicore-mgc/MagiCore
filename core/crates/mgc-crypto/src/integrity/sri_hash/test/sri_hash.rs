//! SRI value compatibility tests — Test tương thích value SRI.
//! Preserve split grammar, empty-part acceptance and display output — Giữ grammar split, chấp nhận phần rỗng và output display.

use crate::{blake3_signer::Blake3Hasher, integrity::SriHash};

#[test]
fn parses_and_displays_two_parts() {
    let value = SriHash::parse("blake3-YWJjMTIz").expect("test operation must succeed");
    assert_eq!(value.algorithm, "blake3");
    assert_eq!(value.hash, "YWJjMTIz");
    assert_eq!(value.to_string(), "blake3-YWJjMTIz");
}

#[test]
fn retains_empty_parts_and_rejects_extra_separators() {
    let empty = SriHash::parse("-").expect("test operation must succeed");
    assert_eq!(empty.algorithm, "");
    assert_eq!(empty.hash, "");
    assert!(SriHash::parse("blake3-a-b").is_err());
}

#[test]
fn creates_base64_sri_from_blake3_digest() {
    let digest = Blake3Hasher::hash_string("sri type");
    let sri = SriHash::from_blake3(&digest);
    assert_eq!(sri.algorithm, "blake3");
    assert_eq!(sri.hash, digest.to_base64());
}
