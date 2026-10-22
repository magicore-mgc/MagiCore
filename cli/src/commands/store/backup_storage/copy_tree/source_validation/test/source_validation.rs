//! Source tree validation cases.
//! Ca xác thực source tree.

use super::super::validate_present_source_tree;
use std::fs;

#[test]
fn accepts_a_regular_source_tree() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join("nested")).unwrap();
    fs::write(temp.path().join("nested/file"), b"bytes").unwrap();
    assert!(validate_present_source_tree(temp.path()).is_ok());
}

#[test]
fn refuses_a_missing_source_tree() {
    let temp = tempfile::tempdir().unwrap();
    assert!(validate_present_source_tree(&temp.path().join("missing")).is_err());
}

#[cfg(unix)]
#[test]
fn refuses_external_link_during_preflight() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let external = temp.path().join("external");
    fs::create_dir(&source).unwrap();
    fs::create_dir(&external).unwrap();
    fs::write(external.join("payload"), b"outside").unwrap();
    symlink(external.join("payload"), source.join("external-link")).unwrap();

    assert!(validate_present_source_tree(&source).is_err());
}

#[cfg(unix)]
#[test]
fn refuses_raw_link_path_that_escapes_and_reenters() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    fs::create_dir_all(source.join("target")).unwrap();
    fs::create_dir(source.join("sub")).unwrap();
    symlink("sub/../../source/target", source.join("escape-and-return")).unwrap();

    assert!(validate_present_source_tree(&source).is_err());
}
