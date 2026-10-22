//! Tests accepted and rejected Cargo archive package roots.
//! Kiểm thử root package Cargo được chấp nhận và bị từ chối.

use super::super::super::test_support::tar_gz_file;
use super::super::*;
#[test]
fn crate_extraction_uses_the_expected_package_root() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    let archive = tar_gz_file("mgcfixture-1.0.0/Cargo.toml", b"[package]\n");

    extract_crate_tar_gz(&archive, root.path(), "mgcfixture-1.0.0")
        .expect("test operation must succeed");

    assert!(root.path().join("mgcfixture-1.0.0/Cargo.toml").is_file());
}

#[test]
fn crate_extraction_refuses_wrong_package_root_before_writing() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    let archive = tar_gz_file("othercrate-1.0.0/Cargo.toml", b"[package]\n");

    let error = extract_crate_tar_gz(&archive, root.path(), "mgcfixture-1.0.0")
        .expect_err("test operation must fail");

    assert!(error.to_string().contains("outside expected root"));
    assert_eq!(
        std::fs::read_dir(root.path())
            .expect("test operation must succeed")
            .count(),
        0
    );
}
