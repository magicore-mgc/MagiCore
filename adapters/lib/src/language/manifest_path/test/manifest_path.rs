//! Tests for regular project manifest paths.
//! Test đường dẫn manifest project dạng regular.

use super::super::is_regular_manifest;
use std::fs;

#[test]
fn regular_file_is_accepted_and_directory_is_not() {
    let project = tempfile::tempdir().expect("temporary project");
    let manifest = project.path().join("package.json");
    fs::write(&manifest, "{}").expect("write regular manifest");
    assert!(is_regular_manifest(&manifest));
    assert!(!is_regular_manifest(project.path()));
}

#[cfg(unix)]
#[test]
fn symlink_manifest_is_not_followed() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("temporary project");
    let target = project.path().join("external.json");
    let link = project.path().join("package.json");
    fs::write(&target, "{}").expect("write external target");
    symlink(&target, &link).expect("create manifest symlink");

    assert!(!is_regular_manifest(&link));
}
