//! Tests for safe package input opening — test mở input package an toàn.

use super::entry_writer::open_package_input;
use std::fs;
use std::path::Path;

#[cfg(unix)]
#[test]
fn final_symlink_is_rejected_without_opening_its_target() {
    let root = tempfile::tempdir().expect("create package fixture directory");
    let outside = tempfile::tempdir().expect("create outside fixture directory");
    let target = outside.path().join("secret");
    let link = root.path().join("README.md");
    fs::write(&target, "private").expect("write secret fixture");
    std::os::unix::fs::symlink(&target, &link).expect("create final symlink fixture");

    let error =
        open_package_input(&link, Path::new("README.md")).expect_err("reject final symlink input");

    assert_eq!(
        error.to_string(),
        "Package input must be a regular file: README.md"
    );
    assert_eq!(fs::read(&target).expect("read secret fixture"), b"private");
}

#[test]
fn directory_input_is_rejected_as_non_regular() {
    let root = tempfile::tempdir().expect("create directory input fixture");
    let error = open_package_input(root.path(), Path::new("folder"))
        .expect_err("reject directory as package input");

    assert_eq!(
        error.to_string(),
        "Package input must be a regular file: folder"
    );
}
