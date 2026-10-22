// Keep cache misses distinct from open failures — Phân biệt cache miss với lỗi mở file.
#[test]
fn missing_tarball_returns_none() {
    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();
    let id = PackageId::parse("absent-package@1.0.0").unwrap();

    assert!(cache.open_tarball(&id).unwrap().is_none());
}
