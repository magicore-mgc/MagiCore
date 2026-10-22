//! Parser boundary tests; no filesystem or network required.
//! Test biên parser; không cần filesystem hay mạng.

#![allow(clippy::unwrap_used)]

use super::{parse_file_header, parse_range};

#[test]
fn rejects_path_traversal_in_file_headers() {
    let mut lines = vec!["--- a/../outside.txt".to_string()]
        .into_iter()
        .peekable();
    let error = parse_file_header(&mut lines, "---").expect_err("test operation must fail");
    assert_eq!(
        error.to_string(),
        "unsafe patch path: path traversal or absolute paths are forbidden: ../outside.txt"
    );
}

#[test]
fn parses_unified_diff_ranges() {
    assert_eq!(
        parse_range("-7,3").expect("test operation must succeed"),
        (7, 3)
    );
}

#[test]
fn defaults_omitted_range_length_to_one() {
    assert_eq!(
        parse_range("+4").expect("test operation must succeed"),
        (4, 1)
    );
}

#[test]
fn rejects_non_numeric_ranges() {
    assert!(parse_range("-x,1").is_err());
}
