//! PEM loader rejection tests.
//! Test từ chối PEM không hợp lệ.

use super::*;

#[test]
fn missing_custom_ca_preserves_read_error_context() {
    let directory = tempfile::tempdir().expect("temporary certificate directory");
    let path = directory.path().join("missing-ca.pem");
    let error = for_each_ca_cert(&path.to_string_lossy(), |_| Ok(()))
        .expect_err("missing CA bundle must be rejected");
    assert!(error.to_string().starts_with("read CA bundle:"));
}

#[test]
fn malformed_custom_ca_preserves_parse_error_context() {
    let directory = tempfile::tempdir().expect("temporary certificate directory");
    let path = directory.path().join("invalid-ca.pem");
    std::fs::write(&path, "not a certificate").expect("write invalid CA bundle");
    let error = for_each_ca_cert(&path.to_string_lossy(), |_| Ok(()))
        .expect_err("malformed CA bundle must be rejected");
    assert!(error.to_string().starts_with("parse CA cert:"));
}

#[test]
fn malformed_certificate_and_private_key_are_rejected() {
    let directory = tempfile::tempdir().expect("temporary certificate directory");
    let cert = directory.path().join("invalid-cert.pem");
    let key = directory.path().join("invalid-key.pem");
    std::fs::write(&cert, "not a certificate").expect("write invalid certificate");
    std::fs::write(&key, "not a private key").expect("write invalid private key");
    assert!(load_certs(&cert.to_string_lossy()).is_err());
    assert!(load_private_key(&key.to_string_lossy()).is_err());
}
