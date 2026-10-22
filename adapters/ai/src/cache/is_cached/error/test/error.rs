//! Cache-presence error case — kiểm tra source I/O của lỗi inspect.

#![cfg(test)]

use super::error::IsCachedError;
use mgc_types::MgError;

#[test]
fn inspection_error_keeps_source_chain() {
    let error = MgError::from(IsCachedError::io(std::io::Error::other(
        "fixture I/O failure",
    )));

    assert!(error.to_string().contains("inspect AI model cache path"));
    assert!(std::error::Error::source(&error).is_some());
}
