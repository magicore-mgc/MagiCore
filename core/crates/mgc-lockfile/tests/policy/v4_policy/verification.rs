// Existing v4 policy test cases: verification.
// Test policy V4 hiện có: verification.

#[test]
fn shared_lock_verifier_dispatches_v4_inline_signature_and_trust() {
    use mgc_lockfile::{VerificationStatus, canonical::write_v4_document};

    let dir = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = dir.path().join("mgc.lock");

    let unsigned = unsigned_doc();
    std::fs::write(&lock_path, write_v4_document(&unsigned).expect("test operation must succeed")).expect("test operation must succeed");
    assert_eq!(
        mgc_lockfile::verify_lockfile_with_trust(&lock_path, &[]).expect("test operation must succeed"),
        VerificationStatus::Unsigned
    );

    let key = mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed");
    let mut signed = unsigned_doc();
    signed.metadata.signature =
        Some(sign_payload_digest(&signed.metadata.lockfile_hash, &key).expect("test operation must succeed"));
    std::fs::write(&lock_path, write_v4_document(&signed).expect("test operation must succeed")).expect("test operation must succeed");
    assert_eq!(
        mgc_lockfile::verify_lockfile(&lock_path).expect("test operation must succeed"),
        VerificationStatus::Valid
    );
    assert_eq!(
        mgc_lockfile::verify_lockfile_with_trust(&lock_path, std::slice::from_ref(&key.key_id))
            .expect("test operation must succeed"),
        VerificationStatus::Valid
    );
    assert_eq!(
        mgc_lockfile::verify_lockfile_with_trust(&lock_path, &[]).expect("test operation must succeed"),
        VerificationStatus::UntrustedKey(key.key_id.clone())
    );

    signed.metadata.signature.as_mut().expect("test operation must succeed").signature = "ed25519-not-base64".to_string();
    std::fs::write(&lock_path, write_v4_document(&signed).expect("test operation must succeed")).expect("test operation must succeed");
    assert!(matches!(
        mgc_lockfile::verify_lockfile_with_trust(&lock_path, std::slice::from_ref(&key.key_id))
            .expect("test operation must succeed"),
        VerificationStatus::InvalidSignature(_)
    ));

    signed.metadata.signature =
        Some(sign_payload_digest(&signed.metadata.lockfile_hash, &key).expect("test operation must succeed"));
    signed.metadata.generator.push_str("-tampered");
    std::fs::write(&lock_path, write_v4_document(&signed).expect("test operation must succeed")).expect("test operation must succeed");
    assert!(matches!(
        mgc_lockfile::verify_lockfile_with_trust(&lock_path, std::slice::from_ref(&key.key_id))
            .expect("test operation must succeed"),
        VerificationStatus::Tampered(_)
    ));
}

#[test]
fn signed_v4_workspace_and_optimizer_tampering_is_detected() {
    use mgc_lockfile::{VerificationStatus, canonical::write_v4_document};

    let dir = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = dir.path().join("mgc.lock");
    let key = mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed");
    let mut signed = unsigned_doc();
    signed.workspace = Some(WorkspaceTopology {
        members: vec!["web".into(), "ai".into()],
        cross_core_edges: vec![CrossEdge {
            from_core: "web".into(),
            to_core: "ai".into(),
            package: "shared-model@1.0.0".into(),
        }],
    });
    signed.optimizer_profile = Some("balanced".into());
    refresh_digest(&mut signed);
    signed.metadata.signature =
        Some(sign_payload_digest(&signed.metadata.lockfile_hash, &key).expect("test operation must succeed"));
    std::fs::write(&lock_path, write_v4_document(&signed).expect("test operation must succeed")).expect("test operation must succeed");
    assert_eq!(
        mgc_lockfile::verify_lockfile(&lock_path).expect("test operation must succeed"),
        VerificationStatus::Valid
    );

    let mut workspace_tampered = signed.clone();
    workspace_tampered
        .workspace
        .as_mut()
        .expect("test operation must succeed")
        .members
        .push("game".into());
    std::fs::write(&lock_path, write_v4_document(&workspace_tampered).expect("test operation must succeed")).expect("test operation must succeed");
    assert!(matches!(
        mgc_lockfile::verify_lockfile(&lock_path).expect("test operation must succeed"),
        VerificationStatus::Tampered(_)
    ));

    let mut optimizer_tampered = signed;
    optimizer_tampered.optimizer_profile = Some("high-performance".into());
    std::fs::write(&lock_path, write_v4_document(&optimizer_tampered).expect("test operation must succeed")).expect("test operation must succeed");
    assert!(matches!(
        mgc_lockfile::verify_lockfile(&lock_path).expect("test operation must succeed"),
        VerificationStatus::Tampered(_)
    ));
}

#[test]
fn shared_lock_verifier_rejects_legacy_sidecar_for_v4() {
    use mgc_lockfile::{VerificationStatus, canonical::write_v4_document};

    let dir = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = dir.path().join("mgc.lock");
    let mut lock = unsigned_doc();
    lock.metadata.signature = Some(
        sign_payload_digest(
            &lock.metadata.lockfile_hash,
            &mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed"),
        )
        .expect("test operation must succeed"),
    );
    std::fs::write(&lock_path, write_v4_document(&lock).expect("test operation must succeed")).expect("test operation must succeed");
    std::fs::write(dir.path().join("mgc.lock.sig"), "stale legacy sidecar").expect("test operation must succeed");

    let status = mgc_lockfile::verify_lockfile(&lock_path).expect("test operation must succeed");
    assert!(matches!(status, VerificationStatus::InvalidSignature(_)));
}

#[test]
fn v4_signature_cannot_claim_another_keys_trusted_id() {
    let key_pair = mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed");
    let mut lock = unsigned_doc();
    let digest = lock.metadata.lockfile_hash.clone();
    let mut block = sign_payload_digest(&digest, &key_pair).expect("test operation must succeed");
    block.key_id = "deadbeefdeadbeef".to_string();
    lock.metadata.signature = Some(block);

    assert!(verify_v4_math(&lock).is_err());
}

#[test]
fn signature_over_wrong_digest_fails() {
    let key_pair = mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed");
    let mut lock = unsigned_doc();
    // Well-formed but unrelated digest: math verifies against the block,
    // yet the block digest disagrees with the payload hash.
    let other = payload_digest(&{
        let mut altered = lock.clone();
        altered.metadata.generator = "mgc/other".to_string();
        altered.payload()
    });
    assert_ne!(other, lock.metadata.lockfile_hash);
    let block = sign_payload_digest(&other, &key_pair).expect("test operation must succeed");
    lock.metadata.signature = Some(block);
    // The block digest disagrees with the file hash: incoherent file,
    // rejected at math stage in every mode (no report to enforce).
    assert!(verify_v4_math(&lock).is_err());
}
