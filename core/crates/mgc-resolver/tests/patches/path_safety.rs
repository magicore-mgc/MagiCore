//! Focused integration cases for resolver patch behavior.
//! Test integration tập trung cho hành vi patch của resolver.

#![allow(clippy::unwrap_used)]

use mgc_resolver::patches::apply_patch;

#[cfg(unix)]
#[test]
fn apply_patch_rejects_symlink_target_escaping_vstore() {
    use std::os::unix::fs::symlink;

    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let vstore = tmp.path().join("vstore");
    std::fs::create_dir(&vstore).expect("test operation must succeed");
    let outside = tmp.path().join("outside.txt");
    std::fs::write(&outside, "keep-me\n").expect("test operation must succeed");
    symlink(&outside, vstore.join("target.txt")).expect("test operation must succeed");
    let patch = tmp.path().join("symlink.patch");
    std::fs::write(
        &patch,
        "--- a/target.txt\n+++ b/target.txt\n@@ -1,1 +1,1 @@\n-keep-me\n+changed\n",
    )
    .expect("test operation must succeed");

    assert!(apply_patch(&vstore, &patch).is_err());
    assert_eq!(
        std::fs::read_to_string(outside).expect("test operation must succeed"),
        "keep-me\n"
    );
}

#[cfg(unix)]
#[test]
fn apply_patch_rejects_parent_path_escape_without_touching_outside_file() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let vstore = tmp.path().join("vstore");
    std::fs::create_dir(&vstore).expect("test operation must succeed");
    let outside = tmp.path().join("outside.txt");
    std::fs::write(&outside, "keep-me\n").expect("test operation must succeed");

    let patch = tmp.path().join("escape.patch");
    std::fs::write(
        &patch,
        "--- a/../outside.txt\n+++ b/../outside.txt\n@@ -1,1 +1,1 @@\n-keep-me\n+owned-by-patch\n",
    )
    .expect("test operation must succeed");

    let result = apply_patch(&vstore, &patch);
    assert!(result.is_err(), "parent traversal must be rejected");
    assert_eq!(
        std::fs::read_to_string(outside).expect("test operation must succeed"),
        "keep-me\n"
    );
}
