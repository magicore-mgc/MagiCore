//! Tests for integrity module
//! Tests cho module integrity

use mgc_crypto::integrity::{IntegrityVerifier, SriHash};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_sri_parse() {
    let sri = SriHash::parse("blake3-YWJjMTIz").expect("test operation must succeed");
    assert_eq!(sri.algorithm, "blake3");
    assert_eq!(sri.hash, "YWJjMTIz");
}

#[test]
fn test_sri_to_string() {
    let sri = SriHash {
        algorithm: "blake3".to_string(),
        hash: "abc123".to_string(),
    };
    // A9 FIX: Use Display trait instead of inherent method
    assert_eq!(sri.to_string(), "blake3-abc123");
    assert_eq!(format!("{}", sri), "blake3-abc123");
}

#[test]
fn test_compute_and_verify() {
    let data = b"hello world";
    let sri = IntegrityVerifier::compute(data);

    assert_eq!(sri.algorithm, "blake3");
    IntegrityVerifier::verify(data, &sri).expect("test operation must succeed");
}

#[test]
fn test_verify_mismatch() {
    let data = b"hello world";
    let sri = IntegrityVerifier::compute(data);

    let wrong_data = b"wrong data";
    assert!(IntegrityVerifier::verify(wrong_data, &sri).is_err());
}

#[test]
fn test_compute_file() {
    let mut tmpfile = NamedTempFile::new().expect("test operation must succeed");
    tmpfile
        .write_all(b"test content")
        .expect("test operation must succeed");
    tmpfile.flush().expect("test operation must succeed");

    let sri = IntegrityVerifier::compute_file(tmpfile.path()).expect("test operation must succeed");
    assert_eq!(sri.algorithm, "blake3");

    IntegrityVerifier::verify_file(tmpfile.path(), &sri).expect("test operation must succeed");
}

#[test]
fn test_unsupported_algorithm() {
    let sri = SriHash {
        algorithm: "sha256".to_string(),
        hash: "abc".to_string(),
    };
    assert!(IntegrityVerifier::verify(b"data", &sri).is_err());
}
