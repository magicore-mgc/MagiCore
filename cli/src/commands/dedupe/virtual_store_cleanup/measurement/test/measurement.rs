//! Dedupe maps shared filesystem errors into its own reportable error type.
//! Dedupe ánh xạ lỗi filesystem dùng chung sang error có thể báo cáo riêng.

use super::super::super::error::VirtualStoreCleanupError;
use super::super::logical_payload_bytes;
use std::fs;

#[test]
fn measurement_refuses_a_regular_file_as_directory_root() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("not-a-directory");
    fs::write(&file, b"payload").unwrap();

    let error = logical_payload_bytes(&file).unwrap_err();

    assert!(matches!(
        error,
        VirtualStoreCleanupError::NotDirectory { .. }
    ));
}
