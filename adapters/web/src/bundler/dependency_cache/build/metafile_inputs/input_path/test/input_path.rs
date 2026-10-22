#![allow(clippy::unwrap_used)]
//! Canonical metafile input-path tests.
//! Test path input metafile canonical.

use std::fs;

use tempfile::tempdir;

use super::super::super::error::MetafileInputsError;
use super::super::validate_input;

#[test]
fn accepts_absolute_and_relative_files_inside_allowed_roots() {
    let project = tempdir().unwrap();
    let package = project.path().join("node_modules/sample");
    let input = package.join("index.js");
    fs::create_dir_all(input.parent().unwrap()).unwrap();
    fs::write(&input, "export default 1;").unwrap();
    let roots = [fs::canonicalize(&package).unwrap()];

    assert!(validate_input("node_modules/sample/index.js", project.path(), &roots).is_ok());
    assert!(validate_input(input.to_str().unwrap(), project.path(), &roots).is_ok());
}

#[test]
fn rejects_parent_traversal_and_missing_input() {
    let project = tempdir().unwrap();
    let package = project.path().join("node_modules/sample");
    fs::create_dir_all(&package).unwrap();
    let private = project.path().join("private.js");
    fs::write(&private, "private").unwrap();
    let roots = [fs::canonicalize(&package).unwrap()];

    assert!(matches!(
        validate_input("private.js", project.path(), &roots),
        Err(MetafileInputsError::OutsideAllowedRoots)
    ));
    assert!(matches!(
        validate_input("missing.js", &package, &roots),
        Err(MetafileInputsError::UnresolvableInput)
    ));
}

#[cfg(unix)]
#[test]
fn rejects_symlink_to_file_outside_allowed_roots() {
    use std::os::unix::fs::symlink;

    let project = tempdir().unwrap();
    let package = project.path().join("node_modules/sample");
    fs::create_dir_all(&package).unwrap();
    let private = project.path().join("private.js");
    fs::write(&private, "private").unwrap();
    symlink(&private, package.join("linked.js")).unwrap();
    let roots = [fs::canonicalize(&package).unwrap()];

    assert!(matches!(
        validate_input("node_modules/sample/linked.js", project.path(), &roots),
        Err(MetafileInputsError::OutsideAllowedRoots)
    ));
}
