//! Signed exception manifest tests — kiểm thử manifest exception có ký.
//!
//! Schema pinning, key trust, revocation, signature, expiry and exact
//! lookup (Spec rev4 §5).
//! (Ghim schema, tin cậy khóa, thu hồi, chữ ký, hạn và tra cứu chính xác
//! (Spec rev4 §5).)

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use mgc_exec::policy_manifest::{
    ManifestEntry, ManifestError, POLICY_MANIFEST_SCHEMA, PolicyManifest, cache_rel_path,
    canonical_bytes, find_exception, verify_manifest,
};

fn test_key() -> ([u8; 32], String) {
    let seed = [7u8; 32];
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed).unwrap();
    let key_id = signer.public_key().to_base64();
    (seed, key_id)
}

fn signed_manifest(expires_at: i64) -> (PolicyManifest, String) {
    let (seed, key_id) = test_key();
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed).unwrap();
    let mut manifest = PolicyManifest {
        schema_version: POLICY_MANIFEST_SCHEMA.to_string(),
        id: "test-next-darwin".to_string(),
        issued_at: 1_000_000,
        expires_at,
        key_id: key_id.clone(),
        revoked_keys: vec![],
        entries: vec![ManifestEntry {
            program: "npm".to_string(),
            args_exact: vec!["--version".to_string()],
            framework: "next".to_string(),
            framework_version: "16.3.8".to_string(),
            os: "darwin".to_string(),
            operation: "dev".to_string(),
            reason: "framework version probe".to_string(),
            expected: "0.0.0".to_string(),
        }],
        signature: String::new(),
    };
    let signature = signer.sign_string(&canonical_bytes(&manifest));
    manifest.signature = signature.to_base64();
    (manifest, key_id)
}

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
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32]).unwrap();
    manifest.signature = signer.sign_string(&canonical_bytes(&manifest)).to_base64();

    assert_eq!(
        verify_manifest(&manifest, &[key_id], &[], 1_500_000),
        Err(ManifestError::InvalidRevocationKey)
    );
}

// Selection applies a signed snapshot's revocation even when it revokes itself.
// Bước chọn áp dụng revocation có ký kể cả khi snapshot tự thu hồi signer.
#[test]
fn self_revocation_snapshot_removes_its_signer_from_active_manifests() {
    let (mut manifest, key_id) = signed_manifest(2_000_000);
    let signer_hex = mgc_exec::policy_manifest::key_id_hex(&key_id).unwrap();
    manifest.revoked_keys = vec![signer_hex];
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32]).unwrap();
    manifest.signature = signer.sign_string(&canonical_bytes(&manifest)).to_base64();

    assert!(
        mgc_exec::policy_manifest::select_active_manifest(
            vec![manifest],
            &[mgc_exec::policy_manifest::key_id_hex(&key_id).unwrap()],
            &[],
            1_500_000,
        )
        .is_none()
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

// Length-prefixed canonical encoding distinguishes argument boundaries.
// Canonical encoding có độ dài phân biệt ranh giới từng argument.
#[test]
fn canonical_bytes_preserve_argument_boundaries() {
    let (mut one_arg, _) = signed_manifest(2_000_000);
    one_arg.entries[0].args_exact = vec!["a b".to_string()];
    let mut two_args = one_arg.clone();
    two_args.entries[0].args_exact = vec!["a".to_string(), "b".to_string()];
    assert_ne!(canonical_bytes(&one_arg), canonical_bytes(&two_args));
}

// Future-dated or non-positive validity windows fail closed.
// Manifest phát hành trong tương lai hoặc cửa sổ hạn không hợp lệ bị chặn.
#[test]
fn manifest_time_window_is_validated() {
    let (seed, key_id) = test_key();
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&seed).unwrap();
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

// Exact lookup matches all six dimensions; anything else misses.
// (Tra cứu chính xác khớp cả sáu chiều; còn lại trượt.)
#[test]
fn exact_lookup() {
    let (manifest, _) = signed_manifest(2_000_000);
    let args = vec!["--version".to_string()];
    let found = find_exception(&manifest, "npm", &args, "next", "16.3.8", "darwin", "dev");
    assert!(found.is_some());
    assert!(find_exception(&manifest, "npm", &args, "next", "16.3.9", "darwin", "dev").is_none());
    assert!(find_exception(&manifest, "npm", &args, "next", "16.3.8", "linux", "dev").is_none());
    assert!(
        find_exception(
            &manifest,
            "npm",
            &["install".to_string()],
            "next",
            "16.3.8",
            "darwin",
            "dev"
        )
        .is_none()
    );
    assert_eq!(cache_rel_path().to_string_lossy(), "policy/exceptions.json");
}

// Key-id hex is the value supplied through user-level trust configuration.
// (Key-id hex là giá trị trong cấu hình trust cấp user.)
#[test]
fn key_id_hex_format() {
    use mgc_exec::policy_manifest::key_id_hex;
    let (_, key_id) = test_key();
    let hex = key_id_hex(&key_id).unwrap();
    assert_eq!(hex.len(), 16);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(key_id_hex("!!!not-base64!!!").is_none());
}

// Rotation: newest valid issued_at wins; revoked signers lose.
// (Rotation: issued_at hợp lệ mới nhất thắng; signer bị thu hồi thua.)
#[test]
fn rotation_selects_newest_unrevoked() {
    use mgc_exec::policy_manifest::select_active_manifest;
    let (mut old, key_id) = signed_manifest(9_000_000);
    old.id = "old".to_string();
    old.issued_at = 1_000_000;
    let old_sig = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32])
        .unwrap()
        .sign_string(&mgc_exec::policy_manifest::canonical_bytes(&old));
    old.signature = old_sig.to_base64();

    let (mut young, _) = signed_manifest(9_000_000);
    young.id = "young".to_string();
    young.issued_at = 2_000_000;
    let young_sig = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32])
        .unwrap()
        .sign_string(&mgc_exec::policy_manifest::canonical_bytes(&young));
    young.signature = young_sig.to_base64();

    let hex = mgc_exec::policy_manifest::key_id_hex(&key_id).unwrap();
    let picked = select_active_manifest(
        vec![old.clone(), young.clone()],
        std::slice::from_ref(&hex),
        &[],
        5_000_000,
    )
    .unwrap();
    assert_eq!(picked.id, "young");

    // A revocation snapshot for the same key kills both.
    // (Snapshot thu hồi cùng khóa giết cả hai.)
    let revoked = [hex.clone()];
    assert!(select_active_manifest(vec![old, young], &revoked, &[hex], 5_000_000).is_none());
}

// Expired signed revocation snapshots still prevent an older signer from
// becoming active again; their exception payload is no longer usable.
// Snapshot thu hồi đã ký vẫn chặn signer cũ sau expiry; payload exception
// của snapshot hết hạn không còn được dùng.
#[test]
fn expired_revocation_snapshot_does_not_revive_old_signer() {
    use mgc_exec::policy_manifest::select_active_manifest;

    let (active, old_public_key) = signed_manifest(9_000_000);
    let old_hex = mgc_exec::policy_manifest::key_id_hex(&old_public_key).unwrap();
    let new_signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[8u8; 32]).unwrap();
    let new_public_key = new_signer.public_key().to_base64();
    let new_hex = mgc_exec::policy_manifest::key_id_hex(&new_public_key).unwrap();
    let mut revocation = PolicyManifest {
        schema_version: POLICY_MANIFEST_SCHEMA.to_string(),
        id: "expired-revocation".to_string(),
        issued_at: 1_000_000,
        expires_at: 2_000_000,
        key_id: new_public_key,
        revoked_keys: vec![old_hex.clone()],
        entries: vec![],
        signature: String::new(),
    };
    revocation.signature = new_signer
        .sign_string(&canonical_bytes(&revocation))
        .to_base64();

    assert!(
        select_active_manifest(
            vec![active, revocation],
            &[old_hex, new_hex],
            &[],
            5_000_000,
        )
        .is_none()
    );
}
