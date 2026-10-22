//! Final symlinks are rejected while regular copies preserve bytes.
//! Symlink cuối phải bị từ chối, copy file thường giữ nguyên bytes.

use super::super::copy_regular_file_no_follow;
use std::fs;
use std::path::Path;

#[test]
fn copies_regular_file_through_staging() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.bin");
    let destination = temp.path().join("destination.bin");
    fs::write(&source, b"source bytes").unwrap();

    copy_regular_file_no_follow(&source, &destination).unwrap();

    assert_eq!(fs::read(destination).unwrap(), b"source bytes");
}

#[cfg(any(unix, windows))]
#[test]
fn refuses_source_symlink_without_copying_its_target() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target.bin");
    let source = temp.path().join("source-link.bin");
    let destination = temp.path().join("destination.bin");
    fs::write(&target, b"secret").unwrap();
    create_file_symlink(&target, &source).unwrap();

    assert!(copy_regular_file_no_follow(&source, &destination).is_err());
    assert!(!destination.exists());
}

#[cfg(any(unix, windows))]
#[test]
fn refuses_destination_symlink_without_overwriting_its_target() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.bin");
    let target = temp.path().join("target.bin");
    let destination = temp.path().join("destination-link.bin");
    fs::write(&source, b"replacement").unwrap();
    fs::write(&target, b"original").unwrap();
    create_file_symlink(&target, &destination).unwrap();

    assert!(copy_regular_file_no_follow(&source, &destination).is_err());
    assert_eq!(fs::read(target).unwrap(), b"original");
}

#[cfg(unix)]
#[test]
fn resolves_a_symlinked_destination_parent_before_staging() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let real_parent = temp.path().join("real-parent");
    let parent_alias = temp.path().join("parent-alias");
    let source = temp.path().join("source.bin");
    fs::create_dir(&real_parent).unwrap();
    fs::write(&source, b"bytes").unwrap();
    symlink(&real_parent, &parent_alias).unwrap();

    copy_regular_file_no_follow(&source, &parent_alias.join("destination.bin")).unwrap();

    assert_eq!(
        fs::read(real_parent.join("destination.bin")).unwrap(),
        b"bytes"
    );
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
