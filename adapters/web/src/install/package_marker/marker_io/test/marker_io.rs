//! Marker persistence and identity predicate contracts.
//! Contract lưu marker và predicate định danh.

use crate::cache::ExtractedPackageMarker;

use super::super::{
    extracted_marker_has_content_signature, extracted_marker_matches_fast,
    read_extracted_package_marker, write_extracted_package_marker,
};

fn marker() -> ExtractedPackageMarker {
    ExtractedPackageMarker {
        schema_version: 3,
        name: "example".to_string(),
        version: "1.0.0".to_string(),
        integrity: Some("sha512-value".to_string()),
        tarball_sha256: "sha256-value".to_string(),
        file_count: 1,
        unpacked_size: 3,
        file_tree_sha256: "tree-value".to_string(),
    }
}

#[test]
fn marker_round_trip_preserves_all_serialized_fields() {
    let directory = tempfile::tempdir().unwrap();
    let expected = marker();

    write_extracted_package_marker(directory.path(), &expected).unwrap();

    assert_eq!(
        read_extracted_package_marker(directory.path()).unwrap(),
        Some(expected)
    );
}

#[test]
fn fast_marker_check_uses_identity_and_content_presence_is_separate() {
    let expected = marker();
    let mut other = expected.clone();
    other.file_tree_sha256 = "different-tree".to_string();

    assert!(extracted_marker_matches_fast(&expected, &other));
    assert!(extracted_marker_has_content_signature(&expected));
    other.file_count = 0;
    assert!(!extracted_marker_has_content_signature(&other));
}
