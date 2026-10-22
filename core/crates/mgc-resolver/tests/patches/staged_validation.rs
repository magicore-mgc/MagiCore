//! Focused integration cases for resolver patch behavior.
//! Test integration tập trung cho hành vi patch của resolver.

#![allow(clippy::unwrap_used)]

use mgc_resolver::patches::apply_patch;

#[test]
fn apply_multiple_files_and_validate_all_before_writing_any_file() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let first = tmp.path().join("first.txt");
    let second = tmp.path().join("second.txt");
    std::fs::write(&first, "first\n").expect("test operation must succeed");
    std::fs::write(&second, "second\n").expect("test operation must succeed");
    let patch = tmp.path().join("multi-file.patch");
    std::fs::write(
        &patch,
        "--- a/first.txt\n+++ b/first.txt\n@@ -1,1 +1,1 @@\n-first\n+FIRST\n--- a/second.txt\n+++ b/second.txt\n@@ -1,1 +1,1 @@\n-wrong\n+SECOND\n",
    )
    .expect("test operation must succeed");

    assert!(apply_patch(tmp.path(), &patch).is_err());
    assert_eq!(
        std::fs::read_to_string(first).expect("test operation must succeed"),
        "first\n"
    );
    assert_eq!(
        std::fs::read_to_string(second).expect("test operation must succeed"),
        "second\n"
    );
}

#[test]
fn malformed_unicode_hunk_line_returns_error_without_panicking() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    std::fs::write(tmp.path().join("test.txt"), "one\n").expect("test operation must succeed");
    let patch = tmp.path().join("unicode.patch");
    std::fs::write(
        &patch,
        "--- a/test.txt\n+++ b/test.txt\n@@ -1,1 +1,1 @@\nébad\n",
    )
    .expect("test operation must succeed");

    let result = std::panic::catch_unwind(|| apply_patch(tmp.path(), &patch));
    assert!(
        result.is_ok(),
        "malformed Unicode must not panic the process"
    );
    assert!(result.expect("test operation must succeed").is_err());
}

#[test]
fn overflowing_hunk_range_returns_error_without_panicking() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    std::fs::write(tmp.path().join("test.txt"), "one\n").expect("test operation must succeed");
    let patch = tmp.path().join("overflow.patch");
    let patch_text = format!(
        "--- a/test.txt\n+++ b/test.txt\n@@ -2,{} +1,1 @@\n-one\n+ONE\n",
        usize::MAX
    );
    std::fs::write(&patch, patch_text).expect("test operation must succeed");

    let result = std::panic::catch_unwind(|| apply_patch(tmp.path(), &patch));
    assert!(result.is_ok(), "malformed hunk range must not panic");
    assert!(result.expect("test operation must succeed").is_err());
}
