//! Focused run registry package regression tests.
//! Test hồi quy tập trung cho run registry package.

use super::*;

#[test]
fn execution_cwd_is_canonicalized_when_it_is_a_real_directory() {
    let directory = tempfile::tempdir().expect("execution directory");
    assert_eq!(
        canonical_execution_cwd(directory.path()).expect("canonical execution directory"),
        directory.path().canonicalize().expect("canonical path")
    );
}

#[test]
fn execution_cwd_rejects_regular_files() {
    let file = tempfile::NamedTempFile::new().expect("execution file");
    assert!(canonical_execution_cwd(file.path()).is_err());
}

#[cfg(unix)]
#[test]
fn execution_cwd_rejects_symlinks_to_directories() {
    let directory = tempfile::tempdir().expect("execution directory");
    let parent = tempfile::tempdir().expect("symlink parent");
    let link = parent.path().join("execution");
    std::os::unix::fs::symlink(directory.path(), &link).expect("create execution symlink");
    assert!(canonical_execution_cwd(&link).is_err());
}
