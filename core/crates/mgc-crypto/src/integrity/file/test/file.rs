//! Integrity file operation tests — Test thao tác integrity file.
//! Preserve file success and I/O failure behavior — Giữ hành vi file thành công và lỗi I/O.

use std::io::Write;

use crate::{CryptoError, integrity::IntegrityVerifier};

#[test]
fn computes_and_verifies_file_integrity() {
    let mut file = tempfile::NamedTempFile::new().expect("test operation must succeed");
    file.write_all(b"integrity file owner")
        .expect("test operation must succeed");
    file.flush().expect("test operation must succeed");

    let sri = IntegrityVerifier::compute_file(file.path()).expect("test operation must succeed");
    assert_eq!(sri.algorithm, "blake3");
    assert!(IntegrityVerifier::verify_file(file.path(), &sri).is_ok());
}

#[test]
fn preserves_io_error_for_missing_file() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let missing = directory.path().join("missing-sri-input");
    let sri = IntegrityVerifier::compute(b"unused");

    assert!(matches!(
        IntegrityVerifier::verify_file(&missing, &sri),
        Err(CryptoError::IoError(_))
    ));
    assert!(matches!(
        IntegrityVerifier::compute_file(&missing),
        Err(CryptoError::IoError(_))
    ));
}
