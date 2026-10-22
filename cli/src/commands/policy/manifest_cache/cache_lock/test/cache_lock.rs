//! Check that independent handles serialize cache writers.
//! Kiểm tra handle độc lập tuần tự hóa writer cache.

#![allow(clippy::unwrap_used)]

use super::lock_policy_manifest_cache;
use fs2::FileExt;

#[test]
fn policy_cache_lock_serializes_independent_file_handles() {
    let directory = tempfile::tempdir().unwrap();
    let cache_path = directory.path().join("exceptions.json");
    let guard = lock_policy_manifest_cache(&cache_path).unwrap();
    let contender = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(cache_path.with_extension("lock"))
        .unwrap();

    assert!(contender.try_lock_exclusive().is_err());
    drop(guard);
    assert!(contender.try_lock_exclusive().is_ok());
}
