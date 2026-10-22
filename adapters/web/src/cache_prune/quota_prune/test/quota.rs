//! Existing quota-pruning behavior tests.
//! Test hành vi quota-pruning có sẵn.

use super::support::tempdir_real;
use crate::cache::{SharedWebCache, read_shared_cache_pinned_package_roots};
use crate::cache_prune::{directory_size, prune_shared_cache_to_quota};

#[test]
fn test_prune_shared_cache_to_quota_removes_prunable_entries() {
    let dir = tempdir_real().unwrap();
    let cache_dir = dir.path().join("cache").join("react");
    let resolution_dir = dir.path().join("resolutions");
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::create_dir_all(&resolution_dir).unwrap();
    std::fs::write(cache_dir.join("18.2.0.tgz"), vec![b'a'; 1024]).unwrap();
    std::fs::write(resolution_dir.join("graph.json"), vec![b'b'; 1024]).unwrap();

    prune_shared_cache_to_quota(dir.path(), 512, &std::collections::HashSet::new()).unwrap();

    let remaining = directory_size(dir.path());
    assert!(
        remaining <= 512,
        "expected quota prune to reduce cache to <= 512 bytes, got {remaining}"
    );
}

#[test]
fn test_prune_shared_cache_to_quota_does_not_delete_unmarked_package_json_dirs() {
    let dir = tempdir_real().unwrap();
    let nested = dir.path().join("packages").join("manual").join("nested");
    let cache_dir = dir.path().join("cache").join("react");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::write(nested.join("package.json"), "{}").unwrap();
    std::fs::write(cache_dir.join("old.tgz"), vec![b'a'; 1024]).unwrap();

    prune_shared_cache_to_quota(dir.path(), 1, &std::collections::HashSet::new()).unwrap();

    assert!(
        nested.join("package.json").exists(),
        "quota pruning should only delete MagiCore-marked package cache roots"
    );
}

#[test]
fn test_prune_shared_cache_to_quota_keeps_pinned_package_roots() {
    let dir = tempdir_real().unwrap();
    let project = tempdir_real().unwrap();
    let package_root = dir
        .path()
        .join("packages")
        .join("react")
        .join("18.2.0-sha512-demo")
        .join("package");
    let cache_dir = dir.path().join("cache").join("old");
    std::fs::create_dir_all(&package_root).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::write(package_root.join("package.json"), br#"{"name":"react"}"#).unwrap();
    std::fs::write(package_root.join(".magicore-package-root.json"), b"{}").unwrap();
    std::fs::write(package_root.join("index.js"), vec![b'a'; 1024]).unwrap();
    std::fs::write(cache_dir.join("old.tgz"), vec![b'b'; 1024]).unwrap();

    let shared = SharedWebCache {
        root: dir.path().to_path_buf(),
    };
    shared
        .write_project_ref(project.path(), [package_root.clone()])
        .unwrap();
    let pinned = read_shared_cache_pinned_package_roots(dir.path());

    prune_shared_cache_to_quota(dir.path(), 1, &pinned).unwrap();

    assert!(
        package_root.join("index.js").exists(),
        "quota pruning must not remove package roots pinned by project refs"
    );
}
