// Count tarballs beneath the hashed-name namespace.
// Đếm tarball bên dưới namespace tên đã hash.
// Keep cache diagnostics accurate after path-key migration.
// Giữ số liệu chẩn đoán cache đúng sau khi đổi path key.
#[test]
fn tarball_count_includes_hashed_name_layout() {
    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();
    let ordinary = PackageId::parse("lodash@4.17.21").unwrap();
    let scoped = PackageId::parse("@scope/pkg@1.2.3").unwrap();

    cache.cache_tarball(&ordinary, b"ordinary").unwrap();
    cache.cache_tarball(&scoped, b"scoped").unwrap();

    assert_eq!(cache.tarball_count(), 2);
}
