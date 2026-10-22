//! Logical-size error case — xác nhận source I/O được giữ lại.

#![cfg(test)]

use super::error::LogicalSizeError;
use mgc_types::MgError;

#[test]
fn logical_size_io_failure_keeps_operation_and_source_chain() {
    let error = MgError::from(LogicalSizeError::io(
        "read cache directory",
        std::io::Error::other("fixture I/O failure"),
    ));

    assert!(error.to_string().contains("read cache directory"));
    assert!(std::error::Error::source(&error).is_some());
}
