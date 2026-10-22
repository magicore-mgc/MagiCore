// IntegrityHash construction, formatting and path contract tests.
// Test hợp đồng dựng, định dạng và path của IntegrityHash.
use super::super::IntegrityHash;
use std::path::Path;

#[test]
fn from_bytes_matches_blake3_abc_vector() {
    let hash = IntegrityHash::from_bytes(b"abc", false);

    assert_eq!(
        hash.as_hex(),
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
    );
    assert!(!hash.is_executable());
}

#[test]
fn from_hash_str_preserves_digest_and_executable_flag() {
    let hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let hash = IntegrityHash::from_hash_str(hex, true).expect("valid lowercase digest");

    assert_eq!(hash.as_hex(), hex);
    assert!(hash.is_executable());
}

#[test]
fn try_from_keeps_non_executable_default() {
    let hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let hash = IntegrityHash::try_from(hex).expect("valid lowercase digest");

    assert!(!hash.is_executable());
}

#[test]
fn cas_path_uses_two_character_shard_and_exec_extension() {
    let hash = IntegrityHash::from_bytes(b"abc", true);
    let expected = Path::new("store-root")
        .join("files")
        .join("blake3")
        .join(&hash.as_hex()[..2])
        .join(format!("{}.exec", hash.as_hex()));

    assert_eq!(hash.cas_path(Path::new("store-root")), expected);
}

#[test]
fn integrity_string_encodes_the_raw_digest_bytes() {
    use base64::Engine;

    let hash = IntegrityHash::from_bytes(b"abc", false);
    let integrity = hash.to_integrity_str();
    let encoded = integrity
        .strip_prefix("blake3-")
        .expect("integrity algorithm prefix");
    let raw = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .expect("base64 digest");

    assert_eq!(
        raw,
        hex::decode(hash.as_hex()).expect("validated hex digest")
    );
}
