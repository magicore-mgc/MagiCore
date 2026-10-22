//! SHA-256 archive verification cases.
//! Các ca xác minh SHA-256 archive.
#![allow(clippy::unwrap_used)]

use super::super::verify;
use sha2::{Digest, Sha256};

#[test]
fn digest_verification_accepts_matching_bytes_and_rejects_mismatch() {
    let bytes = b"archive digest fixture";
    let digest = hex::encode(Sha256::digest(bytes));
    assert!(verify(&digest, bytes).is_ok());
    assert!(verify(&digest, b"different bytes").is_err());
}
