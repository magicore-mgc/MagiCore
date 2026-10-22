//! Tests for `DlxError` source preservation.

use std::error::Error as _;

use super::super::DlxError;

#[test]
fn current_directory_error_keeps_the_io_source() {
    let error = DlxError::CurrentDirectory(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "current directory unavailable",
    ));

    assert_eq!(error.to_string(), "current directory unavailable");
    assert_eq!(
        error.source().unwrap().to_string(),
        "current directory unavailable"
    );
}
