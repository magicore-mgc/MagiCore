//! Checks that a normal temporary directory is not marked as a reparse point.
//! Kiểm tra directory tạm thông thường không bị đánh dấu là reparse point.

use super::super::windows_reparse_point;
use tempfile::tempdir;

#[test]
fn ordinary_directory_has_no_reparse_attribute() {
    let directory = tempdir().expect("test operation must succeed");
    assert!(!windows_reparse_point(directory.path()));
}
