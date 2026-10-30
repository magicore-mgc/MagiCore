//! Signing resolves to ONE keyring: `mgc trust` is authoritative.
//! Ký chỉ dùng MỘT keyring: `mgc trust` là nguồn chính.

use mgc_crypto::keyring::{KeyPair, Keyring};

use crate::commands::build::post_build::sign::implementation::tests::support::support::Sandbox;
use crate::commands::build::post_build::sign::{
    LEGACY_KEY_ID, SIGNING_KEYRING_ENV, resolve_signing_key, trust_keyring_path,
};

const LEGACY_SEED: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";

/// Write a `mgc trust` keyring; return the signer it holds.
/// Ghi keyring `mgc trust`; trả signer.
fn write_trust_keyring(sandbox: &Sandbox) -> mgc_crypto::Ed25519Signer {
    let path = sandbox.trust_keyring();
    std::fs::create_dir_all(path.parent().expect("test")).expect("test operation must succeed");
    let mut keyring = Keyring::new();
    keyring.add_key(KeyPair::generate().expect("test operation must succeed"));
    keyring.save(&path).expect("test operation must succeed");
    Keyring::load(&path)
        .expect("test operation must succeed")
        .default_key()
        .expect("test operation must succeed")
        .signer()
        .expect("test operation must succeed")
}

fn write_legacy_keyring(sandbox: &Sandbox, seed_hex: &str) {
    std::fs::create_dir_all(sandbox.data_dir()).expect("test operation must succeed");
    std::fs::write(
        sandbox.legacy_keyring(),
        format!("{{\"signing_key\":\"{seed_hex}\"}}"),
    )
    .expect("test operation must succeed");
}

fn public_key_of(signer: &mgc_crypto::Ed25519Signer) -> String {
    signer.public_key().to_base64()
}

#[test]
fn the_resolver_reads_the_same_path_trust_writes() {
    let sandbox = Sandbox::new();
    assert_eq!(
        trust_keyring_path().expect("test operation must succeed"),
        sandbox.trust_keyring()
    );
}

#[test]
fn the_trust_keyring_is_the_source_of_truth_for_build_signing() {
    let sandbox = Sandbox::new();
    let expected = write_trust_keyring(&sandbox);

    let key = resolve_signing_key(&sandbox.data_dir()).expect("trust keyring must resolve");

    assert_eq!(key.source, sandbox.trust_keyring());
    assert_eq!(key.public_key_b64, public_key_of(&expected));
    assert!(key.conflict.is_none());
}

#[test]
fn a_legacy_data_dir_keyring_still_signs_when_trust_has_none() {
    let sandbox = Sandbox::new();
    write_legacy_keyring(&sandbox, LEGACY_SEED);

    let key = resolve_signing_key(&sandbox.data_dir()).expect("legacy keyring must resolve");

    assert_eq!(key.source, sandbox.legacy_keyring());
    assert_eq!(key.key_id, LEGACY_KEY_ID);
}

#[test]
fn the_trust_keyring_wins_and_the_other_one_is_reported_as_a_conflict() {
    let sandbox = Sandbox::new();
    let expected = write_trust_keyring(&sandbox);
    write_legacy_keyring(&sandbox, LEGACY_SEED);

    let key = resolve_signing_key(&sandbox.data_dir()).expect("trust keyring must win");

    assert_eq!(key.source, sandbox.trust_keyring());
    assert_eq!(key.public_key_b64, public_key_of(&expected));
    assert_eq!(
        key.conflict.as_deref(),
        Some(sandbox.legacy_keyring().as_path())
    );
}

#[test]
fn an_explicit_keyring_path_overrides_both_locations() {
    let sandbox = Sandbox::new();
    write_trust_keyring(&sandbox);
    let explicit = sandbox.root().join("project-keyring.json");
    let seed = [3u8; 32];
    std::fs::write(
        &explicit,
        format!("{{\"signing_key\":\"{}\"}}", hex::encode(seed)),
    )
    .expect("test operation must succeed");
    sandbox.pin_keyring_env(&explicit);

    let key = resolve_signing_key(&sandbox.data_dir()).expect("explicit keyring must resolve");

    assert_eq!(key.source, explicit);
    assert_eq!(
        key.public_key_b64,
        public_key_of(&mgc_crypto::Ed25519Signer::from_seed(&seed).expect("test"))
    );
    assert_eq!(
        key.conflict.as_deref(),
        Some(sandbox.trust_keyring().as_path())
    );
    assert_eq!(SIGNING_KEYRING_ENV, "MGC_SIGNING_KEYRING");
}

#[test]
fn no_keyring_anywhere_reports_no_key_with_the_trust_instruction() {
    let sandbox = Sandbox::new();

    let error = resolve_signing_key(&sandbox.data_dir()).expect_err("no key must be reported");

    assert!(error.to_string().contains("mgc trust init"), "{error}");
}

#[test]
fn an_unusable_trust_keyring_is_an_error_not_a_fallback_to_the_legacy_key() {
    // Silently signing with a different key than the user believes is in effect
    // would invalidate every verifier that pinned the trust key.
    // (Ký im lặng bằng khoá khác khoá người dùng tưởng sẽ làm vô hiệu mọi verifier
    // đã pin khoá trust.)
    let sandbox = Sandbox::new();
    let path = sandbox.trust_keyring();
    std::fs::create_dir_all(path.parent().expect("test")).expect("test operation must succeed");
    std::fs::write(&path, "{\"keys\":[],\"default_key_id\":null}")
        .expect("test operation must succeed");
    // The keyring loader refuses insecure modes; a hand-written fixture must
    // carry the same owner-only mode `mgc trust init` writes.
    // (Loader từ chối mode không an toàn; fixture viết tay phải có cùng mode
    // owner-only như `mgc trust init` ghi.)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
            .expect("test operation must succeed");
    }
    write_legacy_keyring(&sandbox, LEGACY_SEED);

    let error = resolve_signing_key(&sandbox.data_dir()).expect_err("an empty trust keyring fails");

    assert!(error.to_string().contains("no default key"), "{error}");
}

#[test]
fn a_short_legacy_seed_is_reported_precisely() {
    let sandbox = Sandbox::new();
    write_legacy_keyring(&sandbox, "deadbeef");

    let error = resolve_signing_key(&sandbox.data_dir()).expect_err("a short seed must fail");

    assert!(error.to_string().contains("32 bytes"), "{error}");
}
