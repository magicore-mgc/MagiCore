//! Regression tests for the public SHA-256 verification behavior.
//! Test hồi quy cho hành vi xác minh SHA-256 công khai.

#![allow(clippy::unwrap_used)]

use super::verify_patch_integrity;

#[test]
fn verify_patch_integrity_ok() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let patch = tmp.path().join("test.patch");
    std::fs::write(&patch, "test content").expect("test operation must succeed");
    let sha = {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(b"test content");
        hex::encode(h.finalize())
    };
    assert!(verify_patch_integrity(&patch, &sha).expect("test operation must succeed"));
}

#[test]
fn verify_patch_integrity_fail() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let patch = tmp.path().join("test.patch");
    std::fs::write(&patch, "test content").expect("test operation must succeed");
    assert!(!verify_patch_integrity(&patch, "wrong").expect("test operation must succeed"));
}

#[test]
fn verify_patch_integrity_sri_prefixed() {
    // `mgc patch add` records SRI form (`sha256-<hex>`) — verification
    // must accept it (bare-hex-only comparison rejected every added patch).
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let patch = tmp.path().join("test.patch");
    std::fs::write(&patch, "test content").expect("test operation must succeed");
    let sha = {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(b"test content");
        format!("sha256-{}", hex::encode(h.finalize()))
    };
    assert!(verify_patch_integrity(&patch, &sha).expect("test operation must succeed"));
}
