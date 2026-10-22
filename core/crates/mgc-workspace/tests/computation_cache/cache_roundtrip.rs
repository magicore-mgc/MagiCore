// Cache roundtrip and invalidation regression — kiểm thử hồi quy lưu và vô hiệu hóa cache

#[test]
fn test_computation_caching_roundtrip_and_invalidation() {
    let tmp = tempfile::tempdir().unwrap();
    let pkg_root = tmp.path().join("packages/my-lib");
    std::fs::create_dir_all(&pkg_root).unwrap();
    std::fs::write(pkg_root.join("index.ts"), "export const a = 1;").unwrap();
    std::fs::write(pkg_root.join("package.json"), "{\"name\": \"my-lib\"}").unwrap();

    let mut dep_hashes = std::collections::BTreeMap::new();
    dep_hashes.insert("core-utils".to_string(), "hash_abc_123".to_string());

    // 1. First use has no saved cache, so rebuilding is required.
    // 1. Lần đầu: chưa có cache -> should_rebuild = true
    let (should_rebuild, src_hash, comp_hash) =
        mgc_workspace::check_package_build_freshness(&pkg_root, &dep_hashes).unwrap();
    assert!(should_rebuild, "Fresh package must require build");

    // 2. Save the cache after a successful build.
    // 2. Lưu cache sau khi build
    let saved =
        mgc_workspace::save_package_build_cache(&pkg_root, src_hash, dep_hashes.clone()).unwrap();
    assert_eq!(saved.composite_hash, comp_hash);

    // 3. Rechecking unchanged files must reuse the cache.
    // 3. Kiểm tra lại ngay: không đổi file -> should_rebuild = false (⚡ CACHED)
    let (should_rebuild_after, _, _) =
        mgc_workspace::check_package_build_freshness(&pkg_root, &dep_hashes).unwrap();
    assert!(!should_rebuild_after, "Unchanged package must be cached");

    // 4. Editing index.ts must invalidate the cached result.
    // 4. Sửa file index.ts -> cache bị invalidate -> should_rebuild = true
    std::fs::write(pkg_root.join("index.ts"), "export const a = 2; // modified").unwrap();
    let (should_rebuild_modified, _, _) =
        mgc_workspace::check_package_build_freshness(&pkg_root, &dep_hashes).unwrap();
    assert!(
        should_rebuild_modified,
        "Modified package must trigger rebuild"
    );

    // 5. Changing a dependency hash must invalidate the composite result.
    // 5. Nếu dependency đổi hash -> composite hash đổi -> should_rebuild = true
    let saved_modified = mgc_workspace::save_package_build_cache(
        &pkg_root,
        mgc_workspace::compute_package_source_hash(&pkg_root).unwrap(),
        dep_hashes.clone(),
    )
    .unwrap();
    assert_eq!(
        saved_modified.source_hash,
        mgc_workspace::compute_package_source_hash(&pkg_root).unwrap()
    );

    let mut new_dep_hashes = dep_hashes.clone();
    new_dep_hashes.insert("core-utils".to_string(), "hash_xyz_999".to_string());
    let (should_rebuild_dep_changed, _, _) =
        mgc_workspace::check_package_build_freshness(&pkg_root, &new_dep_hashes).unwrap();
    assert!(
        should_rebuild_dep_changed,
        "Changed upstream dependency must trigger rebuild"
    );
}
