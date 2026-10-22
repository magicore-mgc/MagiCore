//! Typed environment error contracts.
//! Hợp đồng lỗi typed riêng của môi trường.

use super::super::EnvironmentError;
use std::error::Error;

#[test]
fn environment_error_preserves_io_source() {
    let error = EnvironmentError::CreateDirectory(std::io::Error::other("permission denied"));

    assert!(error.to_string().contains("permission denied"));
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("permission denied")
    );
}

#[test]
fn setup_and_cleanup_error_keeps_both_failures_visible() {
    let error = EnvironmentError::CleanupAfterSetup {
        setup: Box::new(EnvironmentError::CreateDirectory(std::io::Error::other(
            "setup denied",
        ))),
        cleanup: Box::new(EnvironmentError::RemoveDirectory {
            path: std::path::PathBuf::from("guarded"),
            source: std::io::Error::other("cleanup denied"),
        }),
    };

    let message = error.to_string();
    assert!(message.contains("setup denied"));
    assert!(message.contains("cleanup denied"));
    assert!(error.source().is_some());
}
