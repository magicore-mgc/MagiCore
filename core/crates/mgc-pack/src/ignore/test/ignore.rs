#![allow(clippy::unwrap_used)]
//! Integration tests for package file selection — test tích hợp chọn file package.

mod support;

#[path = "../include_rules/test/include_rules.rs"]
mod include_rules;
#[path = "../walk_config/test/walk_config.rs"]
mod walk_config;

use mgc_pack::ignore::select_files;
use support::write;

#[test]
fn returns_sorted_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root, "package.json", "{}");
    write(root, "b.js", "x");
    write(root, "a.js", "x");

    let files = select_files(root).unwrap();
    let names: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted);
}
