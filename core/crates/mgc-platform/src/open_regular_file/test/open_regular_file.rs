//! Tests regular-file acceptance and final-link rejection.
//! Kiểm thử chấp nhận file thường và từ chối link cuối.

use super::super::super::error::OpenRegularFileError;
use super::super::{classify_open_error, open_regular_file_no_follow};
use std::io::Read;

#[test]
fn opens_and_reads_a_regular_file() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("payload.bin");
    std::fs::write(&path, b"payload").unwrap();

    let mut file = open_regular_file_no_follow(&path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    assert_eq!(bytes.as_slice(), b"payload");
}

#[test]
fn rejects_a_directory() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("directory");
    std::fs::create_dir(&path).unwrap();

    assert!(matches!(
        open_regular_file_no_follow(&path),
        Err(OpenRegularFileError::NotRegular)
    ));
}

#[cfg(unix)]
#[test]
fn opened_handle_keeps_the_original_file_after_path_replacement() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("payload.bin");
    let moved = temp.path().join("opened.bin");
    let replacement = temp.path().join("replacement.bin");
    std::fs::write(&path, b"original").unwrap();
    std::fs::write(&replacement, b"replacement").unwrap();
    let mut file = open_regular_file_no_follow(&path).unwrap();
    std::fs::rename(&path, &moved).unwrap();
    std::os::unix::fs::symlink(&replacement, &path).unwrap();

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    assert_eq!(bytes.as_slice(), b"original");
}

#[cfg(unix)]
#[test]
fn rejects_a_final_symlink() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let link = temp.path().join("link");
    std::fs::write(&target, b"target").unwrap();
    std::os::unix::fs::symlink(&target, &link).unwrap();

    assert!(matches!(
        open_regular_file_no_follow(&link),
        Err(OpenRegularFileError::Symlink)
    ));
}

#[cfg(unix)]
#[test]
fn classifies_a_raced_no_follow_symlink_open_error() {
    let source = std::io::Error::from_raw_os_error(libc::ELOOP);

    assert!(matches!(
        classify_open_error(source),
        OpenRegularFileError::Symlink
    ));
}

#[cfg(windows)]
#[test]
fn rejects_a_final_reparse_file() {
    use std::os::windows::fs::symlink_file;

    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let link = temp.path().join("link");
    std::fs::write(&target, b"target").unwrap();
    symlink_file(&target, &link).unwrap();

    assert!(matches!(
        open_regular_file_no_follow(&link),
        Err(OpenRegularFileError::Symlink)
            | Err(OpenRegularFileError::OpenedNotRegular)
            | Err(OpenRegularFileError::ReparsePoint)
    ));
}

#[cfg(not(any(unix, windows)))]
#[test]
fn fails_closed_when_no_nofollow_primitive_exists() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("payload.bin");
    std::fs::write(&path, b"payload").unwrap();

    assert!(matches!(
        open_regular_file_no_follow(&path),
        Err(OpenRegularFileError::UnsupportedPlatform)
    ));
}
