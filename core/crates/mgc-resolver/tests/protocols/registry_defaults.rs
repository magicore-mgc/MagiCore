//! Public integration tests for shared registry defaults.
//! Test integration public cho mặc định registry dùng chung.

#![allow(clippy::unwrap_used)]
use mgc_resolver::protocols::{NpmProtocol, RegistryProtocol, ResolvedEntry};

#[test]
fn default_verify_sha256_mismatch_fails_closed() {
    let entry = ResolvedEntry {
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        deps: vec![],
        artifact_url: String::new(),
        sha256: "deadbeef".to_string(),
        extra_markers: vec![],
    };
    let err = NpmProtocol
        .verify(&entry, b"actual bytes")
        .expect_err("test operation must fail");
    assert!(matches!(err, mgc_types::MgError::Integrity(_)), "{err:?}");
}

#[test]
fn default_verify_empty_declared_sha256_fails_closed() {
    // V1.2 zero-trust (D0): an artifact no registry hash vouches for is
    // never installed — the old fail-open (`Ok(())` on empty) was a
    // supply-chain hole, now closed at the shared default.
    let entry = ResolvedEntry {
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        deps: vec![],
        artifact_url: String::new(),
        sha256: String::new(),
        extra_markers: vec![],
    };
    let err = NpmProtocol
        .verify(&entry, b"bytes")
        .expect_err("test operation must fail");
    assert!(matches!(err, mgc_types::MgError::Integrity(_)), "{err:?}");
    assert!(
        err.to_string().contains("without digest"),
        "the error must say why: {err}"
    );
}

#[test]
fn default_store_ref_matches_blake3_cas_layout() {
    let bytes = b"hello native registry";
    let expected_hex = blake3::hash(bytes).to_hex().to_string();
    let expected = format!("files/blake3/{}/{}", &expected_hex[..2], expected_hex);
    assert_eq!(NpmProtocol.store_ref(bytes), expected);
}
