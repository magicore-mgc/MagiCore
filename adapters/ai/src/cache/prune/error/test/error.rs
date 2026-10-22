//! Prune error cases — xác nhận lỗi I/O giữ context và source gốc.

#![cfg(test)]

use super::error::PruneError;
use mgc_types::MgError;

#[test]
fn filesystem_failure_preserves_operation_and_source_chain() {
    let error = MgError::from(PruneError::io(
        "read cache directory",
        std::io::Error::other("fixture I/O failure"),
    ));

    assert!(error.to_string().contains("read cache directory"));
    assert!(std::error::Error::source(&error).is_some());
}
