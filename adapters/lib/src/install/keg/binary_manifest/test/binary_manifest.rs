#![allow(clippy::unwrap_used)]

use super::super::*;

fn signed(entry: BinaryEntry, trust: &mgc_crypto::KeyPair) -> BinaryManifest {
    let signer = trust.signer().unwrap();
    BinaryManifest {
        signature: signer.sign(manifest_message(&entry).as_bytes()).to_base64(),
        public_key: trust.public_key.to_base64(),
        entry,
    }
}

fn entry() -> BinaryEntry {
    BinaryEntry {
        name: "mgc-probe-bin".to_string(),
        version: "1.0.0".to_string(),
        target: "aarch64-apple-darwin".to_string(),
        url: "https://example.invalid/mgc-probe-bin".to_string(),
        blake3: "0".repeat(64),
        executable: true,
    }
}

fn trust_key(pair: &mgc_crypto::KeyPair) -> Vec<String> {
    vec![pair.public_key.to_base64()]
}

/// A manifest signed by a pinned key verifies.
/// (Manifest ký bởi key đã pin thì verify được.)
#[test]
fn signed_manifest_with_pinned_key_verifies() {
    let pair = mgc_crypto::KeyPair::generate().unwrap();
    let manifest = signed(entry(), &pair);
    verify_manifest(&manifest, &trust_key(&pair)).expect("pinned key must verify");
}

/// E-3: without a pinned key nothing is installable (fail closed).
/// (E-3: không có key pin thì không cài được gì — fail-closed.)
#[test]
fn manifest_without_trust_key_is_rejected() {
    let pair = mgc_crypto::KeyPair::generate().unwrap();
    let manifest = signed(entry(), &pair);
    let error = verify_manifest(&manifest, &[]).expect_err("no trust key must fail closed");
    assert!(matches!(error, KegError::ManifestUntrusted { .. }));
}

/// E-3: tampering any field of the signed entry invalidates the signature.
/// (E-3: sửa bất kỳ trường nào của entry đã ký đều làm mất hiệu lực chữ ký.)
#[test]
fn tampered_url_or_digest_is_rejected() {
    let pair = mgc_crypto::KeyPair::generate().unwrap();
    let original = signed(entry(), &pair);

    let mut swapped_url = original.clone();
    swapped_url.entry.url = "https://attacker.invalid/mgc-probe-bin".to_string();
    assert!(verify_manifest(&swapped_url, &trust_key(&pair)).is_err());

    let mut swapped_digest = original.clone();
    swapped_digest.entry.blake3 = "f".repeat(64);
    assert!(verify_manifest(&swapped_digest, &trust_key(&pair)).is_err());
}

/// E-3: a manifest signed by an unpinned key is rejected even if internally
/// consistent, so a stranger cannot publish binaries for this project.
/// (E-3: manifest ký bởi key không được pin vẫn bị từ chối dù tự nhất quán,
/// nên người lạ không publish được binary cho project này.)
#[test]
fn manifest_from_unknown_signer_is_rejected() {
    let trusted = mgc_crypto::KeyPair::generate().unwrap();
    let stranger = mgc_crypto::KeyPair::generate().unwrap();
    let manifest = signed(entry(), &stranger);
    let error = verify_manifest(&manifest, &trust_key(&trusted)).expect_err("unknown signer");
    assert!(matches!(error, KegError::ManifestUntrusted { .. }));
}

/// The message covers every signed field, so two entries never collide.
/// (Message bao mọi trường đã ký nên hai entry không bao giờ trùng nhau.)
#[test]
fn manifest_message_covers_every_field() {
    let base = entry();
    let mut other = base.clone();
    other.target = "x86_64-unknown-linux-gnu".to_string();
    assert_ne!(manifest_message(&base), manifest_message(&other));
    assert!(manifest_message(&base).starts_with(MANIFEST_DOMAIN));
}
