use crate::policy_manifest::test::support::{signed_manifest, test_key};
use crate::policy_manifest::{
    ManifestError, POLICY_MANIFEST_SCHEMA, PolicyManifest, canonical_bytes, verify_manifest,
};

// A well-formed manifest verifies against its key before expiry.
// (Manifest đúng verify được với khóa của nó trước hạn.)
#[test]
fn valid_manifest_verifies() {
    let (manifest, key_id) = signed_manifest(2_000_000);
    assert_eq!(
        verify_manifest(&manifest, &[key_id], &[], 1_500_000),
        Ok(())
    );
}

// Unknown schema fails closed even with a valid signature.
// (Schema lạ fail-closed dù chữ ký đúng.)
#[test]
fn unknown_schema_fails_closed() {
    let (mut manifest, key_id) = signed_manifest(2_000_000);
    manifest.schema_version = "policy-exception-v99".to_string();
    assert_eq!(
        verify_manifest(&manifest, &[key_id], &[], 1_500_000),
        Err(ManifestError::UnknownSchema(
            "policy-exception-v99".to_string()
        ))
    );
}

// Untrusted keys and revoked keys are refused.
// (Khóa không tin và khóa bị thu hồi đều bị từ chối.)
#[test]
fn trust_and_revocation() {
    let (manifest, _) = signed_manifest(2_000_000);
    assert_eq!(
        verify_manifest(&manifest, &["other-key".to_string()], &[], 1_500_000),
        Err(ManifestError::UntrustedKey)
    );
    let (manifest, key_id) = signed_manifest(2_000_000);
    let revoked = [key_id.clone()];
    assert_eq!(
        verify_manifest(
            &manifest,
            std::slice::from_ref(&key_id),
            &revoked,
            1_500_000
        ),
        Err(ManifestError::RevokedKey)
    );
}

// Revoked key ids use lowercase 16-hex BLAKE3 ids, never a silently ignored shape.
// Key bị thu hồi phải có dạng BLAKE3 16 ký tự hex chữ thường.
#[test]
fn malformed_revocation_key_id_fails_closed() {
    let (mut manifest, key_id) = signed_manifest(2_000_000);
    manifest.revoked_keys = vec!["ABCDEF0123456789".to_string()];
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32])
        .expect("valid test fixture");
    manifest.signature = signer.sign_string(&canonical_bytes(&manifest)).to_base64();

    assert_eq!(
        verify_manifest(&manifest, &[key_id], &[], 1_500_000),
        Err(ManifestError::InvalidRevocationKey)
    );
}

// Tampering with any covered field breaks the signature.
// (Sửa bất kỳ field được phủ nào cũng gãy chữ ký.)
#[test]
fn tamper_breaks_signature() {
    let (mut manifest, key_id) = signed_manifest(2_000_000);
    manifest.entries[0].args_exact = vec!["install".to_string()];
    assert_eq!(
        verify_manifest(&manifest, &[key_id], &[], 1_500_000),
        Err(ManifestError::BadSignature)
    );
}

// Past expiry is refused (offline revocation delay is bounded by expiry).
// (Hết hạn bị từ chối (độ trễ thu hồi offline bị giới hạn bởi expiry).)
#[test]
fn expiry_is_enforced() {
    let (manifest, key_id) = signed_manifest(2_000_000);
    assert_eq!(
        verify_manifest(&manifest, &[key_id], &[], 2_000_000),
        Err(ManifestError::Expired)
    );
    assert_eq!(
        verify_manifest(
            &manifest,
            std::slice::from_ref(&manifest.key_id),
            &[],
            2_000_001,
        ),
        Err(ManifestError::Expired)
    );
}

// Future-dated or non-positive validity windows fail closed.
// Manifest phát hành trong tương lai hoặc cửa sổ hạn không hợp lệ bị chặn.
#[test]
fn manifest_time_window_is_validated() {
    let (seed, key_id) = test_key();
    let signer =
        mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed).expect("valid test fixture");
    let mut manifest = PolicyManifest {
        schema_version: POLICY_MANIFEST_SCHEMA.to_string(),
        id: "future".to_string(),
        issued_at: 2_000_000,
        expires_at: 3_000_000,
        key_id: key_id.clone(),
        revoked_keys: vec![],
        entries: vec![],
        signature: String::new(),
    };
    manifest.signature = signer.sign_string(&canonical_bytes(&manifest)).to_base64();
    assert_eq!(
        verify_manifest(&manifest, std::slice::from_ref(&key_id), &[], 1_500_000),
        Err(ManifestError::NotYetValid)
    );
    manifest.expires_at = manifest.issued_at;
    manifest.signature = signer.sign_string(&canonical_bytes(&manifest)).to_base64();
    assert_eq!(
        verify_manifest(&manifest, &[key_id], &[], 1_500_000),
        Err(ManifestError::InvalidTimeWindow)
    );
}
