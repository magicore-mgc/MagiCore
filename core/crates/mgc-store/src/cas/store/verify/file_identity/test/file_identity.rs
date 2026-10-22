//! Tests identity from an opened Unix handle and memo disablement elsewhere.
//! Kiểm thử identity từ handle Unix và tắt memo ở nền tảng khác.

use super::super::file_identity;
use mgc_platform::open_regular_file_no_follow;

#[cfg(unix)]
#[test]
fn returns_a_stable_identity_for_a_regular_file() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("payload.bin");
    std::fs::write(&path, b"payload").unwrap();

    let file = open_regular_file_no_follow(&path).unwrap();
    assert!(file_identity(&file.metadata().unwrap()).is_some());
}

#[cfg(windows)]
#[test]
fn disables_identity_memoization_on_windows() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("payload.bin");
    std::fs::write(&path, b"payload").unwrap();

    let file = open_regular_file_no_follow(&path).unwrap();
    assert!(file_identity(&file.metadata().unwrap()).is_none());
}

#[cfg(not(any(unix, windows)))]
#[test]
fn disables_identity_memoization_on_unsupported_platforms() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("payload.bin");
    std::fs::write(&path, b"payload").unwrap();
    let file = std::fs::File::open(&path).unwrap();

    assert!(file_identity(&file.metadata().unwrap()).is_none());
}
