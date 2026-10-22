//! Check cache locking and reject oversized bytes before writing.
//! Kiểm tra khóa cache và từ chối bytes quá lớn trước khi ghi.

#![allow(clippy::unwrap_used)]

use super::{MAX_POLICY_MANIFEST_BYTES, PolicyManifestCacheError, verify_and_cache};

#[test]
fn oversized_manifest_is_rejected_without_cache_write() {
    let directory = tempfile::tempdir().unwrap();
    let cache_path = directory.path().join("exceptions.json");
    let bytes = vec![b' '; MAX_POLICY_MANIFEST_BYTES + 1];

    assert!(matches!(
        verify_and_cache(&bytes, &[], &cache_path),
        Err(PolicyManifestCacheError::OversizedManifest)
    ));
    assert!(!cache_path.exists());
}

#[test]
fn typed_cache_errors_keep_display_and_source() {
    let rejection = PolicyManifestCacheError::UpdateRejected("snapshot rolled back".to_string());
    assert_eq!(
        rejection.to_string(),
        "signed policy manifest update rejected: snapshot rolled back"
    );

    let parse_error = serde_json::from_slice::<serde_json::Value>(b"{").unwrap_err();
    let invalid_json = PolicyManifestCacheError::InvalidJson(parse_error);
    assert!(std::error::Error::source(&invalid_json).is_some());
}
