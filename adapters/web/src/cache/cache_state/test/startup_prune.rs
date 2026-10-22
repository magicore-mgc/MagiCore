//! Shared-cache startup pruning behavior test.
//! Test hành vi prune khi khởi động shared cache.

use super::support::tempdir_real;
use crate::cache::{SharedWebCache, write_shared_cache_prune_stamp};

#[test]
fn test_maybe_prune_skips_quota_scan_when_gc_not_due() {
    let dir = tempdir_real().unwrap();
    let cache_dir = dir.path().join("cache").join("react");
    std::fs::create_dir_all(&cache_dir).unwrap();
    let tarball_path = cache_dir.join("18.2.0.tgz");
    std::fs::write(&tarball_path, vec![b'a'; 1024]).unwrap();

    write_shared_cache_prune_stamp(dir.path()).unwrap();

    let shared = SharedWebCache {
        root: dir.path().to_path_buf(),
    };
    shared.maybe_prune();

    assert!(
        tarball_path.exists(),
        "fresh gc stamp should skip quota pruning on adapter startup"
    );
}
