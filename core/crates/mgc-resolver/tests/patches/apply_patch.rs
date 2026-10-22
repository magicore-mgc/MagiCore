//! Successful and context-mismatch integration cases for patch application.
//! Test integration cho áp dụng patch thành công và sai lệch context.

#![allow(clippy::unwrap_used)]

use mgc_resolver::patches::apply_patch;

#[test]
fn apply_simple_patch() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let vstore = tmp.path();

    // Create original file
    let orig = vstore.join("test.txt");
    std::fs::write(&orig, "line1\nline2\nline3\n").expect("test operation must succeed");

    // Create patch
    let patch = vstore.join("test.patch");
    std::fs::write(
        &patch,
        r#"--- a/test.txt
+++ b/test.txt
@@ -1,3 +1,3 @@
 line1
-line2
+LINE2
 line3
"#,
    )
    .expect("test operation must succeed");

    let modified = apply_patch(vstore, &patch).expect("test operation must succeed");
    assert_eq!(modified.len(), 1);
    let content = std::fs::read_to_string(&orig).expect("test operation must succeed");
    assert_eq!(content, "line1\nLINE2\nline3\n");
}

#[test]
fn apply_multiple_hunks_in_one_file() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let target = tmp.path().join("test.txt");
    std::fs::write(&target, "one\ntwo\nthree\nfour\nfive\nsix\n")
        .expect("test operation must succeed");
    let patch = tmp.path().join("multi.patch");
    std::fs::write(
        &patch,
        "--- a/test.txt\n+++ b/test.txt\n@@ -1,1 +1,1 @@\n-one\n+ONE\n@@ -6,1 +6,1 @@\n-six\n+SIX\n",
    )
    .expect("test operation must succeed");

    apply_patch(tmp.path(), &patch).expect("test operation must succeed");
    assert_eq!(
        std::fs::read_to_string(target).expect("test operation must succeed"),
        "ONE\ntwo\nthree\nfour\nfive\nSIX\n"
    );
}

#[test]
fn patch_context_mismatch_fails() {
    let tmp = tempfile::tempdir().expect("test operation must succeed");
    let vstore = tmp.path();

    let orig = vstore.join("test.txt");
    std::fs::write(&orig, "line1\nline2\nline3\n").expect("test operation must succeed");

    let patch = vstore.join("test.patch");
    std::fs::write(
        &patch,
        r#"--- a/test.txt
+++ b/test.txt
@@ -1,3 +1,3 @@
 line1
-lineX
+LINE2
 line3
"#,
    )
    .expect("test operation must succeed");

    let result = apply_patch(vstore, &patch);
    assert!(result.is_err());
    assert!(
        result
            .expect_err("test operation must fail")
            .to_string()
            .contains("context mismatch")
    );
}
