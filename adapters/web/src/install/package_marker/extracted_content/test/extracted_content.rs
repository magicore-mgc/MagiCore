//! Extracted package content and mutation contracts.
//! Contract nội dung package extract và phát hiện thay đổi.

use sha2::{Digest, Sha256};

use crate::cache::ExtractedPackageMarker;
use crate::install::package_marker::{compute_sha256_hex, update_tree_hasher};

use super::super::extracted_content_matches;

fn expected_marker(path: &str, contents: &[u8]) -> ExtractedPackageMarker {
    let mut tree = Sha256::new();
    update_tree_hasher(
        &mut tree,
        path,
        contents.len() as u64,
        false,
        &compute_sha256_hex(contents),
    );
    ExtractedPackageMarker {
        schema_version: 3,
        name: "example".to_string(),
        version: "1.0.0".to_string(),
        integrity: None,
        tarball_sha256: "archive-sha256".to_string(),
        file_count: 1,
        unpacked_size: contents.len() as u64,
        file_tree_sha256: hex::encode(tree.finalize()),
    }
}

#[test]
fn extracted_tree_signature_matches_until_file_content_changes() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("index.js");
    let marker = expected_marker("index.js", b"abc");
    std::fs::write(&path, b"abc").unwrap();

    assert!(extracted_content_matches(directory.path(), &marker).unwrap());
    std::fs::write(&path, b"changed").unwrap();
    assert!(!extracted_content_matches(directory.path(), &marker).unwrap());
}

#[test]
fn extracted_tree_without_a_content_signature_is_not_trusted() {
    let directory = tempfile::tempdir().unwrap();
    let marker = expected_marker("index.js", b"abc");
    let mut fast_marker = marker;
    fast_marker.file_tree_sha256.clear();

    assert!(!extracted_content_matches(directory.path(), &fast_marker).unwrap());
}
