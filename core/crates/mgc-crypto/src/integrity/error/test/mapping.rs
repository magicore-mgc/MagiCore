//! Integrity error compatibility tests — Test tương thích lỗi integrity.
//! Preserve public diagnostic strings — Giữ chuỗi diagnostic public.

use crate::{CryptoError, integrity::error::IntegrityError};

#[test]
fn maps_sri_algorithm_and_mismatch_errors() {
    let invalid: CryptoError = IntegrityError::InvalidSriFormat("bad".to_string()).into();
    let unsupported: CryptoError =
        IntegrityError::UnsupportedAlgorithm("sha256".to_string()).into();
    let unknown: CryptoError = IntegrityError::UnknownAlgorithm("md5".to_string()).into();
    let mismatch: CryptoError = IntegrityError::HashMismatch {
        expected: "expected".to_string(),
        actual: "actual".to_string(),
    }
    .into();

    assert!(
        matches!(invalid, CryptoError::Blake3Failed(message) if message == "invalid SRI format: bad")
    );
    assert!(
        matches!(unsupported, CryptoError::Blake3Failed(message) if message == "algorithm not supported: sha256 (use blake3)")
    );
    assert!(
        matches!(unknown, CryptoError::Blake3Failed(message) if message == "unknown algorithm: md5")
    );
    assert!(
        matches!(mismatch, CryptoError::VerificationFailed(message) if message == "BLAKE3 hash mismatch: expected expected, got actual")
    );
}
