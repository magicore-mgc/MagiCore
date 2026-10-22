//! Cache-size error case — xác nhận root sai dạng có diagnostic typed.

#![cfg(test)]

use super::error::CacheSizeError;
use mgc_types::MgError;

#[test]
fn non_directory_root_has_typed_diagnostic() {
    let error = MgError::from(CacheSizeError::RootNotDirectory);

    assert!(error.to_string().contains("cache root is not a directory"));
    let MgError::Anyhow(cause) = error else {
        panic!("expected typed error boundary");
    };
    assert!(matches!(
        cause.downcast_ref::<CacheSizeError>(),
        Some(CacheSizeError::RootNotDirectory)
    ));
}

#[test]
fn io_error_preserves_operation_and_source_chain() {
    let error = MgError::from(CacheSizeError::io(
        "resolve",
        std::io::Error::other("fixture I/O failure"),
    ));

    assert!(error.to_string().contains("resolve"));
    assert!(std::error::Error::source(&error).is_some());
}
