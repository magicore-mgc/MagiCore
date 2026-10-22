//! Tests refusal of a pre-planted linked extraction parent.
//! Kiểm thử từ chối parent giải nén đã bị gài symlink.

use super::super::super::test_support::tar_gz_file;
use crate::protocols::archive::extract_tar_gz;
#[cfg(unix)]
#[test]
fn tar_extraction_refuses_preexisting_symlink_parent() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("test operation must succeed");
    let outside = tempfile::tempdir().expect("test operation must succeed");
    symlink(outside.path(), root.path().join("nested")).expect("test operation must succeed");
    let archive = tar_gz_file("nested/pwned.txt", b"outside");

    let result = extract_tar_gz(&archive, root.path());

    assert!(result.is_err(), "extractor must refuse a linked parent");
    assert!(
        !outside.path().join("pwned.txt").exists(),
        "tar data must never be written through a pre-existing symlink"
    );
}
