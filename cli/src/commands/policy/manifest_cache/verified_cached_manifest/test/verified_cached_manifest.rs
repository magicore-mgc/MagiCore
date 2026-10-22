//! Malformed cached JSON cannot become a trusted snapshot.
//! JSON cache sai không thể trở thành snapshot tin cậy.

#![allow(clippy::unwrap_used)]

use super::read_verified_cached_manifest;

#[test]
fn malformed_cache_is_ignored() {
    let directory = tempfile::tempdir().unwrap();
    let cache_path = directory.path().join("exceptions.json");
    std::fs::write(&cache_path, b"{}").unwrap();

    assert!(read_verified_cached_manifest(&cache_path, &[]).is_none());
}
