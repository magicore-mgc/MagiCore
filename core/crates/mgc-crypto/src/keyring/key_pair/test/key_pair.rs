//! KeyPair contract tests — Test contract KeyPair.
//! These tests do not print or persist private-key material — Test không in hoặc lưu byte private.

use super::key_pair::{KeyPair, unix_timestamp};

#[test]
fn generated_pair_has_ed25519_public_key_and_key_id() {
    let key_pair = KeyPair::generate().expect("test operation must succeed");
    assert_eq!(key_pair.public_key.0.len(), 32);
    assert!(!key_pair.key_id.is_empty());
}

#[test]
fn signer_reconstructs_the_same_public_key() {
    let key_pair = KeyPair::generate().expect("test operation must succeed");
    let signer = key_pair.signer().expect("test operation must succeed");
    assert_eq!(signer.public_key(), key_pair.public_key);
}

#[test]
fn debug_redacts_private_key_bytes() {
    let key_pair = KeyPair::generate().expect("test operation must succeed");
    let debug = format!("{key_pair:?}");
    let private_bytes = format!("{:?}", key_pair.private_key_pkcs8);
    assert!(debug.contains("REDACTED"));
    assert!(!debug.contains(&private_bytes));
}

#[test]
fn unix_timestamp_rejects_a_clock_before_the_epoch() {
    let before_epoch = std::time::UNIX_EPOCH - std::time::Duration::from_secs(1);
    assert!(unix_timestamp(before_epoch).is_err());
}
