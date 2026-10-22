use super::super::super::test_support::temp_dir_with_npmrc;
use super::super::*;
use crate::commands::config::implementation::set_npmrc;
use std::fs;

#[test]
fn file_value_reads_npmrc_key() {
    let dir = temp_dir_with_npmrc("registry=https://example.com\nfoo=bar\n");
    let val = file_value(&dir.path().join(".npmrc"), "registry").unwrap();
    assert_eq!(val, Some("https://example.com".to_string()));
    let missing = file_value(&dir.path().join(".npmrc"), "missing_key").unwrap();
    assert!(missing.is_none());
}

#[test]
fn file_value_ignores_comments() {
    let dir = temp_dir_with_npmrc("# this is a comment\n;also ignored\nkey=value\n");
    let val = file_value(&dir.path().join(".npmrc"), "key").unwrap();
    assert_eq!(val, Some("value".to_string()));
    // comment line must not be a key
    let comment_val = file_value(&dir.path().join(".npmrc"), "# this is a comment").unwrap();
    assert!(comment_val.is_none());
}

#[cfg(unix)]
#[test]
fn npmrc_read_and_write_reject_symlinks_without_touching_target() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("outside.npmrc");
    let path = dir.path().join(".npmrc");
    fs::write(&target, "_authToken=do-not-touch\n").unwrap();
    let before = fs::read(&target).unwrap();
    symlink(&target, &path).unwrap();

    assert!(file_value(&path, "_authToken").is_err());
    assert!(read_lines(&path).is_err());
    assert!(set_npmrc(&path, "registry", "https://example.test").is_err());
    assert_eq!(fs::read(&target).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn npmrc_read_rejects_fifo_without_blocking() {
    use std::os::unix::fs::FileTypeExt;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".npmrc");
    let status = std::process::Command::new("mkfifo")
        .arg(&path)
        .status()
        .expect("POSIX test runner must provide mkfifo");
    assert!(status.success());

    let read_path = path.clone();
    let result = std::thread::spawn(move || file_value(&read_path, "registry"))
        .join()
        .unwrap();

    assert!(result.is_err());
    assert!(fs::symlink_metadata(path).unwrap().file_type().is_fifo());
}
