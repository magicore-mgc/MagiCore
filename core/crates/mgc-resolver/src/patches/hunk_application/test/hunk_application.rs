//! Hunk application boundary tests using only in-memory text.
//! Test biên áp dụng hunk chỉ dùng văn bản trong bộ nhớ.

#![allow(clippy::unwrap_used)]

use super::apply_hunks;
use crate::patches::types::Hunk;

#[test]
fn applies_a_replacement_and_preserves_trailing_newline() {
    let hunk = Hunk {
        old_start: 2,
        old_len: 1,
        lines: vec![('-', "old".into()), ('+', "new".into())],
    };
    assert_eq!(
        apply_hunks("first\nold\nlast\n", vec![hunk]).expect("test operation must succeed"),
        "first\nnew\nlast\n"
    );
}

#[test]
fn rejects_mismatched_context() {
    let hunk = Hunk {
        old_start: 1,
        old_len: 1,
        lines: vec![('-', "expected".into()), ('+', "next".into())],
    };
    assert!(
        apply_hunks("actual\n", vec![hunk])
            .expect_err("test operation must fail")
            .to_string()
            .contains("context mismatch")
    );
}
