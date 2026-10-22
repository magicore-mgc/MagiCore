//! Behavior tests for the registration_integrity owner.
//! Test hành vi cho owner registration_integrity.

use super::super::*;
use mgc_types::MgError;

#[test]
fn catalog_hash_accepts_sha512_case_insensitively_and_preserves_marker() {
    let marker = validate_catalog_hash("Demo", "1.0.0", "sha512", "AQID")
        .expect("test operation must succeed");

    assert_eq!(marker, "sha512:AQID");
}

#[test]
fn catalog_hash_rejects_wrong_algorithm_and_empty_digest() {
    let wrong_algorithm = validate_catalog_hash("Demo", "1.0.0", "SHA256", "AQID");
    let empty_hash = validate_catalog_hash("Demo", "1.0.0", "SHA512", "");

    assert!(matches!(wrong_algorithm, Err(MgError::Integrity(_))));
    assert!(matches!(empty_hash, Err(MgError::Integrity(_))));
}

#[test]
fn catalog_hash_rejects_invalid_base64() {
    let result = validate_catalog_hash("Demo", "1.0.0", "SHA512", "***");

    assert!(matches!(result, Err(MgError::Integrity(_))));
}
