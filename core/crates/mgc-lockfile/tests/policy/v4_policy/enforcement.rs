// Existing v4 policy test cases: enforcement.
// Test policy V4 hiện có: enforcement.

#[test]
fn tampered_digest_fails_in_every_mode() {
    let mut lock = unsigned_doc();
    lock.metadata.lockfile_hash = "blake3-tampered".to_string();
    let report = verify_v4_math(&lock).expect("test operation must succeed");
    assert!(!report.digest_ok);
    for policy in [
        LockPolicyMode::Off,
        LockPolicyMode::Warn,
        LockPolicyMode::Require,
    ] {
        assert!(enforce_policy(&report, policy, &[]).is_err());
    }
}

#[test]
fn unsigned_lock_per_policy() {
    let lock = unsigned_doc();
    let report = verify_v4_math(&lock).expect("test operation must succeed");
    assert!(report.digest_ok);
    assert!(!report.signed);
    assert!(enforce_policy(&report, LockPolicyMode::Off, &[]).is_ok());
    assert!(enforce_policy(&report, LockPolicyMode::Warn, &[]).is_ok());
    assert!(enforce_policy(&report, LockPolicyMode::Require, &[]).is_err());
}

#[test]
fn signed_lock_verifies_and_trust_roots_gate_require() {
    let key_pair = mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed");
    let mut lock = unsigned_doc();
    let digest = lock.metadata.lockfile_hash.clone();
    let block = sign_payload_digest(&digest, &key_pair).expect("test operation must succeed");
    assert_eq!(block.digest, digest);
    assert_eq!(block.algorithm, "ed25519");
    lock.metadata.signature = Some(block);
    let report = verify_v4_math(&lock).expect("test operation must succeed");
    assert!(report.digest_ok && report.signed && report.signature_ok);
    // Trusted key passes require; foreign key fails require but passes
    // warn (advisory, never silent).
    assert!(enforce_policy(&report, LockPolicyMode::Require, &[key_pair.key_id]).is_ok());
    assert!(matches!(
        enforce_policy(
            &report,
            LockPolicyMode::Require,
            &["deadbeefdeadbeef".to_string()]
        ),
        Err(LockfileError::UntrustedKey(_))
    ));
    assert!(enforce_policy(&report, LockPolicyMode::Warn, &[]).is_ok());
}

