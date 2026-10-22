//! Regular-file read and metadata rejection cases.
//! Kiểm tra đọc file thường và từ chối metadata không phù hợp.

use super::{read_regular_text, reject_non_regular};
use std::fs;

#[test]
fn reads_a_regular_utf8_file() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("manifest.toml");
    fs::write(&path, "[package]\nname = 'demo'\n").unwrap();

    assert_eq!(
        read_regular_text(&path, "manifest.toml").unwrap(),
        "[package]\nname = 'demo'\n"
    );
}

#[test]
fn rejects_a_directory_as_a_manifest() {
    let root = tempfile::tempdir().unwrap();
    assert!(reject_non_regular(root.path()).is_err());
}
