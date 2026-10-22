// Keep a safe canonical prerelease filename stable — Giữ filename prerelease chuẩn an toàn ổn định.
// The version round-trips without using path separators — Version round-trip mà không dùng separator path.
#[test]
fn portable_prerelease_versions_keep_legacy_filename() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cache");
    let cache = PackageCache::new(root.clone()).unwrap();
    let package = PackageId::parse("pkg@1.2.3-alpha.1").unwrap();

    assert_eq!(
        cache.tarball_path(&package),
        root.join("pkg").join("1.2.3-alpha.1.tgz")
    );
}
