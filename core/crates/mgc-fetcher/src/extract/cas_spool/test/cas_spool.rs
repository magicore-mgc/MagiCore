//! Covers repeated paths and npm package-root recognition.
//! Kiểm tra path trùng và nhận diện root package npm.

use super::super::super::test_support::gzip_tar_files;

#[test]
fn the_last_duplicate_file_entry_wins() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let archive = gzip_tar_files(&[("pkg/file", b"old"), ("pkg/file", b"new")]);
    let staged = super::super::spool_archive(archive.as_slice(), &root, false)
        .unwrap()
        .unwrap();
    assert_eq!(staged.files.len(), 1);
    assert_eq!(std::fs::read(&staged.files[0].source_path).unwrap(), b"new");
}

#[test]
fn non_package_root_requests_ordinary_staging_fallback() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let archive = gzip_tar_files(&[("other/file", b"content")]);
    assert!(
        super::super::spool_archive(archive.as_slice(), &root, true)
            .unwrap()
            .is_none()
    );
}
