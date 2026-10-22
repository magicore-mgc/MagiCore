#[test]
fn conflicting_tarball_winner_is_not_overwritten() {
    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();
    let package = PackageId::parse("same-package@1.2.3").unwrap();

    cache.cache_tarball(&package, b"verified winner").unwrap();
    let conflict = cache.cache_tarball(&package, b"different bytes");

    assert!(
        conflict.is_err(),
        "different bytes for one package ID must conflict"
    );
    assert_eq!(
        cache.get_tarball(&package).unwrap().unwrap(),
        b"verified winner"
    );
}
