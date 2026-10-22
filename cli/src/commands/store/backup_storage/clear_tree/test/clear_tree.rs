//! Clear refuses symlink roots and removes a link without touching its target.
//! Clear từ chối root symlink và xóa link mà không chạm target.
use super::super::clear_dir;
use std::fs;
use std::path::Path;

#[test]
fn clear_removes_regular_children_and_preserves_root() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("root");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("file.bin"), b"data").unwrap();
    fs::create_dir(root.join("nested")).unwrap();
    clear_dir(&root).unwrap();
    assert!(root.is_dir());
    assert_eq!(fs::read_dir(root).unwrap().count(), 0);
}

#[cfg(any(unix, windows))]
#[test]
fn clear_refuses_symlink_root_without_touching_target() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let root_link = temp.path().join("root-link");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("keep.bin"), b"keep").unwrap();
    create_directory_symlink(&target, &root_link).unwrap();
    assert!(clear_dir(&root_link).is_err());
    assert_eq!(fs::read(target.join("keep.bin")).unwrap(), b"keep");
}

#[cfg(any(unix, windows))]
#[test]
fn clear_removes_child_symlink_without_touching_target() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("root");
    let target = temp.path().join("target");
    fs::create_dir(&root).unwrap();
    fs::create_dir(&target).unwrap();
    fs::write(target.join("keep.bin"), b"keep").unwrap();
    create_directory_symlink(&target, &root.join("link")).unwrap();
    clear_dir(&root).unwrap();
    assert_eq!(fs::read(target.join("keep.bin")).unwrap(), b"keep");
    assert_eq!(fs::read_dir(root).unwrap().count(), 0);
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
