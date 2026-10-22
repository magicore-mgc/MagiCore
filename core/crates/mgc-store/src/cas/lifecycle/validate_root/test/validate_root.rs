//! CAS root validation boundary tests.
//! Test biên cho validate root CAS.

#[test]
fn missing_root_is_validated_without_being_created() {
    let parent = tempfile::tempdir().unwrap();
    let missing = parent.path().join("not-created").join("store");
    super::super::validate_cas_root(&missing).unwrap();
    assert!(!missing.exists());
}
