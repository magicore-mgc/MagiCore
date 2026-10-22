//! Covers the non-standard archive-root fallback exposed by the CAS API.
//! Kiểm tra fallback root archive không chuẩn của API CAS.

use super::super::super::test_support::gzip_tar_file;

#[test]
fn returns_false_when_existing_root_archive_is_not_npm_shaped() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let package_root = root.join("package");
    let store = mgc_store::ContentStore::new(root.join("store/cas")).unwrap();
    let archive = gzip_tar_file("other/file", b"content");
    let result = super::super::extract_tarball_to_cas_and_verify_existing_root(
        archive.as_slice(),
        &package_root,
        &store,
        None,
    )
    .unwrap();
    assert!(!result);
}
