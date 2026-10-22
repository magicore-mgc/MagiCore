// Check the file-backed API at its public boundary — Kiểm tra API dạng file tại public boundary.
#[test]
fn open_cached_tarball_returns_readable_file() {
    use std::io::Read;

    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();
    let id = PackageId::parse("streamed-package@1.0.0").unwrap();
    let bytes = b"file-backed cached tarball";
    cache.cache_tarball(&id, bytes).unwrap();

    let mut file = cache.open_tarball(&id).unwrap().unwrap();
    assert_eq!(file.metadata().unwrap().len(), bytes.len() as u64);
    let mut actual = Vec::new();
    file.read_to_end(&mut actual).unwrap();

    assert_eq!(actual, bytes);
}
