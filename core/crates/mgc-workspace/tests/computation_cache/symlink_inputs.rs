// Symlink-input cache safety regressions — kiểm thử an toàn input symlink của cache

#[cfg(unix)]
#[test]
fn computation_cache_rejects_symlinked_source_file() {
    let tmp = tempfile::tempdir().unwrap();
    let pkg_root = tmp.path().join("package");
    let outside = tmp.path().join("outside.ts");
    std::fs::create_dir_all(&pkg_root).unwrap();
    std::fs::write(&outside, "outside source").unwrap();
    std::os::unix::fs::symlink(&outside, pkg_root.join("source.ts")).unwrap();

    let result = mgc_workspace::compute_package_source_hash(&pkg_root);

    assert!(
        result.is_err(),
        "a symlink must not enter a cached source hash"
    );
}

#[cfg(unix)]
#[test]
fn computation_cache_rejects_symlinked_source_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let pkg_root = tmp.path().join("package");
    let outside = tmp.path().join("outside");
    std::fs::create_dir_all(&pkg_root).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("source.ts"), "outside source").unwrap();
    std::os::unix::fs::symlink(&outside, pkg_root.join("linked")).unwrap();

    let result = mgc_workspace::compute_package_source_hash(&pkg_root);

    assert!(
        result.is_err(),
        "a symlinked directory must not enter a cached source hash"
    );
}

#[cfg(unix)]
#[test]
fn computation_cache_rejects_symlinked_package_root() {
    let tmp = tempfile::tempdir().unwrap();
    let real_root = tmp.path().join("real-package");
    let linked_root = tmp.path().join("linked-package");
    std::fs::create_dir_all(&real_root).unwrap();
    std::fs::write(real_root.join("source.ts"), "source").unwrap();
    std::os::unix::fs::symlink(&real_root, &linked_root).unwrap();

    let result = mgc_workspace::compute_package_source_hash(&linked_root);

    assert!(
        result.is_err(),
        "a symlinked package root must not be cached"
    );
}
