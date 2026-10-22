//! Manifest-reader source-chain and display tests.
//! Test display và chuỗi source của bộ đọc manifest.

#![allow(clippy::unwrap_used)]

use crate::graph::manifest_reader::error::ManifestReadError;

#[test]
fn io_error_keeps_source_and_display_text() {
    let source = std::io::Error::other("permission denied");
    let error = ManifestReadError::Io(source);

    assert_eq!(error.to_string(), "permission denied");
    assert_eq!(
        std::error::Error::source(&error).unwrap().to_string(),
        "permission denied"
    );
}
