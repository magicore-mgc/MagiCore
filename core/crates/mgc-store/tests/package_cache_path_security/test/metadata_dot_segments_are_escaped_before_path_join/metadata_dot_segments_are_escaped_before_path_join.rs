#[test]
fn metadata_dot_segments_are_escaped_before_path_join() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cache");
    let cache = PackageCache::new(root.clone()).unwrap();
    let current = cache.metadata_path(".");
    let parent = cache.metadata_path("..");

    assert_eq!(
        current.strip_prefix(&root).unwrap(),
        std::path::Path::new("~d").join("metadata.json")
    );
    assert_eq!(
        parent.strip_prefix(&root).unwrap(),
        std::path::Path::new("~dd").join("metadata.json")
    );
    cache.cache_metadata("..", b"inside cache root").unwrap();
    assert_eq!(
        std::fs::read(parent).unwrap(),
        b"inside cache root",
        "metadata for a parent-segment input must be written below the cache root"
    );
    assert!(
        !temp.path().join("metadata.json").exists(),
        "metadata writes must not escape to the cache root's parent"
    );
}
