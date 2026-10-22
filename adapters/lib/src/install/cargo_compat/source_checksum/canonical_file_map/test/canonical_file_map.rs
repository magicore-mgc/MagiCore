//! Check canonical source-map containment and matching.
//! Kiểm tra containment và đối chiếu source map canonical.

use super::super::implementation::{canonical_source_file_map, source_files_match_archive};
use std::collections::BTreeMap;

#[test]
fn accepts_identical_maps_for_the_same_source_root() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("lib.rs"), b"source").unwrap();
    let files = BTreeMap::from([("lib.rs".to_string(), "digest".to_string())]);

    let canonical_root = std::fs::canonicalize(temp.path()).unwrap();
    let canonical = canonical_source_file_map(temp.path(), &canonical_root, &files).unwrap();
    assert!(canonical.is_some());
    assert!(source_files_match_archive(temp.path(), &files, &files).unwrap());
}

#[test]
fn rejects_paths_that_escape_the_source_root() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("lib.rs"), b"source").unwrap();
    let expected = BTreeMap::from([("../outside.rs".to_string(), "digest".to_string())]);
    let actual = BTreeMap::from([("lib.rs".to_string(), "digest".to_string())]);

    let canonical_root = std::fs::canonicalize(temp.path()).unwrap();
    assert!(
        canonical_source_file_map(temp.path(), &canonical_root, &expected)
            .unwrap()
            .is_none()
    );
    assert!(!source_files_match_archive(temp.path(), &expected, &actual).unwrap());
}
