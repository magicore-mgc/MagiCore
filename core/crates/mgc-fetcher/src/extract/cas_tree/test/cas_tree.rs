//! Covers symlink rejection during CAS destination preparation.
//! Kiểm tra từ chối symlink khi chuẩn bị đích CAS.

#[cfg(unix)]
#[test]
fn refuses_linked_archive_directory() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let root = base.join("root");
    let outside = base.join("outside");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&outside).unwrap();
    symlink(&outside, root.join("linked")).unwrap();
    let archive = super::super::super::test_support::gzip_tar_file("linked/file", b"blocked");
    let staged = super::super::super::cas_spool::spool_archive(archive.as_slice(), &root, false)
        .unwrap()
        .unwrap();
    let error = super::super::prepare_destination(&root, &staged).unwrap_err();
    assert!(matches!(
        error,
        super::super::super::ExtractionError::ParentSymlink(_)
    ));
    assert!(!outside.join("file").exists());
}
