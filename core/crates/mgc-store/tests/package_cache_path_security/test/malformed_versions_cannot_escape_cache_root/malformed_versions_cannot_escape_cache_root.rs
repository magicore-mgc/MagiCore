// Keep malformed prerelease text inside the cache root — Giữ prerelease lỗi bên trong cache root.
// Parsing preserves this text, so the path owner must contain it.
// Parser giữ nguyên text này nên path owner phải chặn traversal.
#[test]
fn malformed_versions_cannot_escape_cache_root() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cache");
    let cache = PackageCache::new(root.clone()).unwrap();
    let package = PackageId::parse("lodash@1.2.3-../../../../outside").unwrap();
    let tarball = cache.tarball_path(&package);

    assert!(tarball.starts_with(&root));
    assert!(
        !tarball
            .strip_prefix(&root)
            .unwrap()
            .components()
            .any(|part| part == std::path::Component::ParentDir)
    );
    cache.cache_tarball(&package, b"contained").unwrap();
    assert!(tarball.is_file());
    assert!(!temp.path().join("outside.tgz").exists());
}
