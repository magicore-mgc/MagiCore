//! Tests refusal of a symlink as the extraction root.
//! Kiểm thử từ chối symlink được dùng làm root giải nén.

use super::super::super::test_support::tar_gz_file;
use crate::protocols::archive::InstallHookPolicy;
use crate::protocols::archive::tar_extraction::extract_tar_gz;
#[cfg(unix)]
#[test]
fn tar_extraction_refuses_symlink_root() {
    use std::os::unix::fs::symlink;

    let parent = tempfile::tempdir().expect("test operation must succeed");
    let outside = tempfile::tempdir().expect("test operation must succeed");
    let linked_root = parent.path().join("linked-root");
    symlink(outside.path(), &linked_root).expect("test operation must succeed");
    let archive = tar_gz_file("pwned.txt", b"outside");

    assert!(extract_tar_gz(&archive, &linked_root, &InstallHookPolicy::deny()).is_err());
    assert!(!outside.path().join("pwned.txt").exists());
}
