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

#[test]
fn spool_refuses_a_pth_entry_before_staging_any_file() {
    // SEC-3: the litellm 1.107.0 shape — a normal package plus a `.pth`.
    // (SEC-3: hình dạng litellm 1.107.0 — package bình thường kèm `.pth`.)
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let archive = gzip_tar_files(&[
        ("package/index.js", b"module.exports = 1;"),
        ("package/litellm.pth", b"import os\n"),
    ]);
    let error = match super::super::spool_archive(archive.as_slice(), &root, true) {
        Err(error) => error,
        Ok(_) => panic!("pth entry must be refused"),
    };
    assert!(error.to_string().contains("litellm.pth"), "{error}");
    // Nothing may be staged: the error surfaces before any spool file exists.
    // (Không được staging gì: lỗi xuất hiện trước khi spool tạo file nào.)
    let leftovers: Vec<_> = std::fs::read_dir(temp.path())
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name())
        .filter(|name| name.to_string_lossy().starts_with(".mgc-extract-"))
        .collect();
    assert!(leftovers.is_empty(), "spool left {leftovers:?}");
}

#[test]
fn spool_refuses_sitecustomize_and_nuget_scripts() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    for (path, needle) in [
        ("package/sitecustomize.py", "sitecustomize.py"),
        ("package/tools/install.ps1", "install.ps1"),
    ] {
        let archive = gzip_tar_files(&[("package/package.json", b"{}"), (path, b"x")]);
        let error = match super::super::spool_archive(archive.as_slice(), &root, true) {
            Err(error) => error,
            Ok(_) => panic!("{needle} entry must be refused"),
        };
        assert!(error.to_string().contains(needle), "{error}");
    }
}
