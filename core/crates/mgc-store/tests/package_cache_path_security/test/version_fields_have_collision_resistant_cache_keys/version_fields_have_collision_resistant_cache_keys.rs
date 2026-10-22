// Keep distinct version fields from aliasing through display text.
// Tránh field version khác nhau alias qua text hiển thị.
// Deserialized values can have equal display strings but distinct identities.
// Dữ liệu deserialize có thể cùng text hiển thị nhưng identity khác nhau.
#[test]
fn version_fields_have_collision_resistant_cache_keys() {
    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();
    let name = mgc_types::PackageName::new("pkg").unwrap();
    let first = mgc_types::Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: Some("alpha.dev1".to_string()),
        build: None,
        dev: None,
        post: None,
    };
    let second = mgc_types::Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: Some("alpha".to_string()),
        build: None,
        dev: Some(1),
        post: None,
    };
    let first_id = PackageId::new(name.clone(), first.clone());
    let second_id = PackageId::new(name, second.clone());

    assert_eq!(first.to_string(), second.to_string());
    assert_ne!(cache.tarball_path(&first_id), cache.tarball_path(&second_id));
}
