// Preserve ordinary portable package cache paths — Giữ nguyên path cache của package portable thông thường.
// This guards the compatibility path used by common package names — Bài này giữ tương thích cho tên package phổ biến.
#[test]
fn ordinary_package_names_keep_existing_cache_path_layout() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cache");
    let cache = PackageCache::new(root.clone()).unwrap();
    let package = PackageId::parse("lodash@4.17.21").unwrap();

    assert_eq!(
        cache.tarball_path(&package),
        root.join("lodash").join("4.17.21.tgz")
    );
}
