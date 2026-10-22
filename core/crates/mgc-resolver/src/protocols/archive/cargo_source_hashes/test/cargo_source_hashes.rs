//! Tests source hashes relative to a validated Cargo package root.
//! Kiểm thử hash source theo root package Cargo đã xác minh.

use super::super::super::test_support::tar_gz_entries;
use crate::protocols::archive::cargo_source_file_hashes;
use sha2::{Digest, Sha256};
#[test]
fn hashes_only_files_relative_to_the_validated_cargo_root() {
    let manifest = b"[package]\nname = \"fixture\"\n";
    let source = b"pub fn value() -> u8 { 7 }\n";
    let archive = tar_gz_entries(&[
        ("fixture-1.0.0/Cargo.toml", manifest),
        ("fixture-1.0.0/src/lib.rs", source),
    ]);

    let hashes =
        cargo_source_file_hashes(&archive, "fixture-1.0.0").expect("test operation must succeed");

    assert_eq!(hashes.len(), 2);
    assert_eq!(
        hashes
            .get("Cargo.toml")
            .expect("test operation must succeed"),
        &hex::encode(Sha256::digest(manifest))
    );
    assert_eq!(
        hashes
            .get("src/lib.rs")
            .expect("test operation must succeed"),
        &hex::encode(Sha256::digest(source))
    );
}
