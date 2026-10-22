//! Listing error cases — xác nhận context và source filesystem được giữ lại.

#![cfg(test)]

use super::error::ListCachedModelsError;
use mgc_types::MgError;

#[test]
fn io_error_keeps_operation_context_and_source_chain() {
    let error = MgError::from(ListCachedModelsError::io(
        "read cache root",
        std::io::Error::other("fixture I/O failure"),
    ));

    assert!(error.to_string().contains("read cache root"));
    assert!(std::error::Error::source(&error).is_some());
}
