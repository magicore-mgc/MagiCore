#[test]
fn metadata_escape_marker_does_not_alias_literal_names() {
    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();

    assert_ne!(cache.metadata_path("."), cache.metadata_path("~d"));
    assert_ne!(cache.metadata_path(".."), cache.metadata_path("~dd"));
}
