//! Pipe-read error source tests.
//! Test source lỗi đọc pipe.

use super::*;
use std::error::Error;

#[test]
fn stream_read_error_retains_the_original_io_failure() {
    let error = StreamReadError::new(std::io::Error::other("pipe closed"));
    assert_eq!(
        error.to_string(),
        "failed to read child output stream: pipe closed"
    );
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("pipe closed")
    );
}
