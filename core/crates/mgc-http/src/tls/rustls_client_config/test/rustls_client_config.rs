//! rustls trust construction and verifier tests.
//! Test dựng trust rustls và verifier.

use super::*;

#[test]
fn untrusted_verifier_keeps_the_reviewed_signature_scheme_set() {
    let schemes =
        rustls::client::danger::ServerCertVerifier::supported_verify_schemes(&NoVerify::new());
    assert_eq!(
        schemes,
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ED25519,
        ]
    );
}
