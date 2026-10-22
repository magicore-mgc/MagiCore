use crate::policy_manifest::test::support::signed_manifest;
use crate::policy_manifest::{
    POLICY_MANIFEST_SCHEMA, PolicyManifest, canonical_bytes, select_active_manifest,
};

// Selection applies a signed snapshot's revocation even when it revokes itself.
// Bước chọn áp dụng revocation có ký kể cả khi snapshot tự thu hồi signer.
#[test]
fn self_revocation_snapshot_removes_its_signer_from_active_manifests() {
    let (mut manifest, key_id) = signed_manifest(2_000_000);
    let signer_hex = crate::policy_manifest::key_id_hex(&key_id).expect("valid test fixture");
    manifest.revoked_keys = vec![signer_hex];
    let signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32])
        .expect("valid test fixture");
    manifest.signature = signer.sign_string(&canonical_bytes(&manifest)).to_base64();

    assert!(
        crate::policy_manifest::select_active_manifest(
            vec![manifest],
            &[crate::policy_manifest::key_id_hex(&key_id).expect("valid test fixture")],
            &[],
            1_500_000,
        )
        .is_none()
    );
}

// Rotation: newest valid issued_at wins; revoked signers lose.
// (Rotation: issued_at hợp lệ mới nhất thắng; signer bị thu hồi thua.)
#[test]
fn rotation_selects_newest_unrevoked() {
    let (mut old, key_id) = signed_manifest(9_000_000);
    old.id = "old".to_string();
    old.issued_at = 1_000_000;
    let old_sig = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32])
        .expect("valid test fixture")
        .sign_string(&crate::policy_manifest::canonical_bytes(&old));
    old.signature = old_sig.to_base64();

    let (mut young, _) = signed_manifest(9_000_000);
    young.id = "young".to_string();
    young.issued_at = 2_000_000;
    let young_sig = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[7u8; 32])
        .expect("valid test fixture")
        .sign_string(&crate::policy_manifest::canonical_bytes(&young));
    young.signature = young_sig.to_base64();

    let hex = crate::policy_manifest::key_id_hex(&key_id).expect("valid test fixture");
    let picked = select_active_manifest(
        vec![old.clone(), young.clone()],
        std::slice::from_ref(&hex),
        &[],
        5_000_000,
    )
    .expect("valid test fixture");
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
    let (active, old_public_key) = signed_manifest(9_000_000);
    let old_hex = crate::policy_manifest::key_id_hex(&old_public_key).expect("valid test fixture");
    let new_signer = mgc_crypto::ed25519_signer::Ed25519Signer::from_seed(&[8u8; 32])
        .expect("valid test fixture");
    let new_public_key = new_signer.public_key().to_base64();
    let new_hex = crate::policy_manifest::key_id_hex(&new_public_key).expect("valid test fixture");
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
