use crate::credential_security::TokenDigest;

#[test]
fn token_v1_domain_is_stable_and_stored_digest_is_not_a_raw_token() {
    let digest = TokenDigest::from_secret("client-token");
    assert_eq!(
        digest.as_str(),
        "mgc-token-sha256-v1:c93859beadda2a704189bcfd6802390d9efd36021785ae52f6764139e5f0d1d5"
    );
    assert!(TokenDigest::valid_stored(digest.as_str()));
    assert!(!TokenDigest::valid_stored("client-token"));
    assert_ne!(
        TokenDigest::from_secret(digest.as_str()).as_str(),
        digest.as_str()
    );
    assert!(!TokenDigest::valid_stored(&format!("{}z", digest.as_str())));
}
