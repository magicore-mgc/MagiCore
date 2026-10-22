//! In-memory SRI verification tests — Test verify SRI trong bộ nhớ.
//! Preserve successful, mismatch, unsupported and unknown paths — Giữ nhánh thành công, mismatch, unsupported và unknown.

use crate::{
    CryptoError,
    integrity::{IntegrityVerifier, SriHash},
};

#[test]
fn verifies_matching_digest_and_rejects_mismatch() {
    let sri = IntegrityVerifier::compute(b"integrity verify");
    assert!(IntegrityVerifier::verify(b"integrity verify", &sri).is_ok());
    assert!(matches!(
        IntegrityVerifier::verify(b"changed", &sri),
        Err(CryptoError::VerificationFailed(message)) if message.starts_with("BLAKE3 hash mismatch:")
    ));
}

#[test]
fn preserves_unsupported_and_unknown_algorithm_errors() {
    let unsupported = SriHash {
        algorithm: "sha256".to_string(),
        hash: "abc".to_string(),
    };
    let unknown = SriHash {
        algorithm: "md5".to_string(),
        hash: "abc".to_string(),
    };

    assert!(matches!(
        IntegrityVerifier::verify(b"data", &unsupported),
        Err(CryptoError::Blake3Failed(message)) if message == "algorithm not supported: sha256 (use blake3)"
    ));
    assert!(matches!(
        IntegrityVerifier::verify(b"data", &unknown),
        Err(CryptoError::Blake3Failed(message)) if message == "unknown algorithm: md5"
    ));
}
