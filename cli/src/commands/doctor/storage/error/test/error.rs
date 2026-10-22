//! Verifies the storage error retains its operating-system source.
//! Kiểm tra lỗi storage vẫn giữ source của hệ điều hành.

use std::error::Error as _;

use super::super::StorageError;

#[test]
fn probe_error_preserves_io_source_and_path_context() {
    let source = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test denied");
    let error = StorageError::CreateWriteProbe {
        directory: "/protected/store".into(),
        source,
    };

    assert!(error.to_string().contains("/protected/store"));
    assert_eq!(error.source().unwrap().to_string(), "test denied");
}
