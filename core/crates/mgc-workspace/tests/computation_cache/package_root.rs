// Package-root cache safety regression — kiểm thử an toàn root package cache

#[test]
fn computation_cache_rejects_missing_package_root() {
    let tmp = tempfile::tempdir().unwrap();
    let missing_root = tmp.path().join("missing-package");

    let result = mgc_workspace::compute_package_source_hash(&missing_root);

    assert!(
        result.is_err(),
        "a missing source tree must not hash as empty"
    );
}
