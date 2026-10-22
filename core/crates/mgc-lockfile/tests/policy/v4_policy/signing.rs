// Existing v4 policy test cases: signing.
// Test policy V4 hiện có: signing.

#[test]
fn malformed_digest_is_rejected_at_sign_time() {
    let key_pair = mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed");
    assert!(sign_payload_digest("blake3-!!!not-base64!!!", &key_pair).is_err());
    assert!(sign_payload_digest("no-prefix-at-all", &key_pair).is_err());
}

