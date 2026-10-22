// Reject links at the final cache component — Từ chối link ở thành phần cuối cache.
#[cfg(unix)]
#[test]
fn final_symlink_is_rejected() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(temp.path().join("cache")).unwrap();
    let id = PackageId::parse("linked-package@1.0.0").unwrap();
    let target = temp.path().join("outside.tgz");
    std::fs::write(&target, b"outside cache root").unwrap();
    let link = cache.tarball_path(&id);
    std::fs::create_dir_all(link.parent().unwrap()).unwrap();
    symlink(target, &link).unwrap();

    assert!(cache.open_tarball(&id).is_err());
}
