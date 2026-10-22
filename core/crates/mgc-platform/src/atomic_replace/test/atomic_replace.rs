//! Verifies replacement of an existing entry and staging consumption.
//! Xác minh thay entry cũ và tiêu thụ staging.

use super::super::{
    AtomicReplaceError, atomic_replace_file, atomic_replace_file_with_write_through,
};

#[test]
fn replacement_publishes_new_bytes_and_consumes_staging_path() {
    let directory = tempfile::tempdir().unwrap();
    let staging = directory.path().join("staging.tmp");
    let destination = directory.path().join("content-address");
    std::fs::write(&staging, b"verified bytes").unwrap();
    std::fs::write(&destination, b"old bytes").unwrap();

    atomic_replace_file(&staging, &destination).unwrap();

    assert_eq!(std::fs::read(&destination).unwrap(), b"verified bytes");
    assert!(!staging.exists());
}

#[test]
fn caller_selected_write_through_mode_still_replaces_atomically() {
    let directory = tempfile::tempdir().unwrap();
    let staging = directory.path().join("staging.tmp");
    let destination = directory.path().join("content-address");
    std::fs::write(&staging, b"verified bytes").unwrap();
    std::fs::write(&destination, b"old bytes").unwrap();

    atomic_replace_file_with_write_through(&staging, &destination, false).unwrap();

    assert_eq!(std::fs::read(&destination).unwrap(), b"verified bytes");
    assert!(!staging.exists());
}

#[test]
fn replacement_error_keeps_paths_and_io_source() {
    let directory = tempfile::tempdir().unwrap();
    let staging = directory.path().join("missing.tmp");
    let destination = directory.path().join("content-address");

    let error = atomic_replace_file_with_write_through(&staging, &destination, false).unwrap_err();

    let AtomicReplaceError::Replace {
        staging: actual_staging,
        destination: actual_destination,
        ..
    } = &error;
    assert_eq!(actual_staging, &staging);
    assert_eq!(actual_destination, &destination);
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(error.io_source().kind(), std::io::ErrorKind::NotFound);
}
