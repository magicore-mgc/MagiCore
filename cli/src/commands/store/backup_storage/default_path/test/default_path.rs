//! Default path keeps the existing store_backup naming.
//! Path mặc định giữ quy ước tên store_backup hiện hành.
use super::super::{default_path_under, timestamp};
use std::fs;
use std::path::Path;

#[test]
fn timestamp_is_decimal_and_default_path_uses_existing_prefix() {
    assert!(timestamp().bytes().all(|byte| byte.is_ascii_digit()));
    let temp = tempfile::tempdir().unwrap();
    let path = default_path_under(temp.path()).unwrap();
    assert!(path.starts_with(temp.path().join(".magicore")));
    assert!(
        path.file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("store_backup_")
    );
    assert!(!temp.path().join(".magicore").exists());
}

#[cfg(any(unix, windows))]
#[test]
fn default_path_refuses_symlinked_magicore_root() {
    let temp = tempfile::tempdir().unwrap();
    let external = temp.path().join("external");
    let home = temp.path().join("home");
    let base = home.join(".magicore");
    fs::create_dir(&external).unwrap();
    fs::create_dir(&home).unwrap();
    create_directory_symlink(&external, &base).unwrap();
    assert!(default_path_under(&home).is_err());
    assert_eq!(fs::read_dir(external).unwrap().count(), 0);
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
