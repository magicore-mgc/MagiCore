#![allow(clippy::unwrap_used)]
//! Staging error compatibility tests. / Kiểm thử tương thích lỗi staging.

use super::super::*;
use mgc_types::MgError;

#[test]
fn staging_io_error_retains_source_and_legacy_store_message() {
    let error = StagingError::CreateDirectory(std::io::Error::other("parent denied"));
    assert!(std::error::Error::source(&error).is_some());

    let legacy = MgError::from(error);
    let MgError::Store(message) = legacy else {
        panic!("staging failures must remain store errors");
    };
    assert_eq!(message, "parent denied");
}
