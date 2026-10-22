//! Tests digest text conversion and length validation.
//! Kiểm tra chuyển đổi digest sang text và xác thực độ dài.
//! Kiểm tra chuyển đổi văn bản digest và xác thực độ dài.

use crate::blake3_signer::{Blake3Hash, Blake3Hasher};

#[test]
fn hex_and_base64_round_trip_preserve_digest() {
    let digest = Blake3Hasher::hash_string("type encoding test");
    assert_eq!(
        Blake3Hash::from_hex(&digest.to_hex()).expect("test operation must succeed"),
        digest
    );
    assert_eq!(
        Blake3Hash::from_base64(&digest.to_base64()).expect("test operation must succeed"),
        digest
    );
    assert_eq!(digest.to_string(), digest.to_hex());
}

#[test]
fn parsers_reject_invalid_lengths() {
    assert!(Blake3Hash::from_hex("00").is_err());
    assert!(Blake3Hash::from_base64("AA==").is_err());
}
