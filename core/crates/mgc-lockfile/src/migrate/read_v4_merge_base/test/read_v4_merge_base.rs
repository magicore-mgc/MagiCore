//! Checks the no-base case for a not-yet-created lockfile.
//! Kiểm tra trường hợp lockfile chưa tồn tại nên chưa có merge base.

use super::super::read_v4_merge_base;

#[test]
fn missing_lockfile_has_no_legacy_rows() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let (base, had_legacy_rows) =
        read_v4_merge_base(&temp.path().join("missing.lock")).expect("test operation must succeed");
    assert!(base.is_none());
    assert!(!had_legacy_rows);
}
