//! Covers stable error text and source-chain retention.
//! Kiểm tra text lỗi ổn định và giữ chuỗi source.

use std::io;

#[test]
fn preserves_archive_error_messages() {
    let path = std::path::PathBuf::from("../escape");
    let error = super::super::ExtractionError::UnsafeArchivePath(path);
    assert_eq!(error.to_string(), "unsafe tar entry path: ../escape");
    let limit = super::super::ExtractionError::EntryLimit;
    assert_eq!(
        limit.to_string(),
        "tar archive exceeds the 16384 entry limit"
    );
}

#[test]
fn retains_io_error_as_source() {
    let error = super::super::ExtractionError::from(io::Error::other("fixture"));
    assert_eq!(
        std::error::Error::source(&error).unwrap().to_string(),
        "fixture"
    );
}
