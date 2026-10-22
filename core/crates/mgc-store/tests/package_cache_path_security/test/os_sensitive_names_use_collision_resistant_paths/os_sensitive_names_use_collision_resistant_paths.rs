// Separate names that alias on case-insensitive or Windows filesystems.
// Tách tên alias trên filesystem không phân biệt hoa thường hoặc Windows.
// Hash non-portable names while keeping lowercase paths stable.
// Hash tên không portable, đồng thời giữ path chữ thường ổn định.
#[test]
fn os_sensitive_names_use_collision_resistant_paths() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cache");
    let cache = PackageCache::new(root.clone()).unwrap();
    let upper = PackageId::parse("Foo@1.2.3").unwrap();
    let lower = PackageId::parse("foo@1.2.3").unwrap();
    let reserved = PackageId::parse("nul@1.2.3").unwrap();
    let reserved_with_extension = PackageId::parse("nul.txt@1.2.3").unwrap();
    let trailing_dot = PackageId::parse("name.@1.2.3").unwrap();
    let without_dot = PackageId::parse("name@1.2.3").unwrap();

    assert_ne!(cache.tarball_path(&upper), cache.tarball_path(&lower));
    assert_ne!(
        cache.tarball_path(&reserved),
        cache.tarball_path(&reserved_with_extension)
    );
    assert_ne!(
        cache.tarball_path(&trailing_dot),
        cache.tarball_path(&without_dot)
    );
    assert_eq!(
        cache.tarball_path(&lower),
        root.join("foo").join("1.2.3.tgz")
    );
}
