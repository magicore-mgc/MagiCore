//! Ensures each local I/O operation keeps its established diagnostic label.
//! Đảm bảo mỗi thao tác I/O giữ nguyên nhãn diagnostic đã có.

use super::super::{CasWriteOperation, map_io_error};
use crate::cas::store::StoreError;
use std::io;
use std::path::Path;

#[test]
fn io_error_mapping_preserves_operation_text_and_path() {
    let cases = [
        (CasWriteOperation::Read, "read failed: disk unavailable"),
        (CasWriteOperation::Write, "write failed: disk unavailable"),
        (CasWriteOperation::Flush, "flush failed: disk unavailable"),
        (
            CasWriteOperation::FlushAndSync,
            "flush/fsync failed: disk unavailable",
        ),
        (
            CasWriteOperation::SetPermissions,
            "set permissions failed: disk unavailable",
        ),
        (
            CasWriteOperation::ParentDirectorySync,
            "fsync parent directory failed: disk unavailable",
        ),
    ];
    let path = Path::new("store/cas/tmp/object");

    for (operation, expected_message) in cases {
        let error = map_io_error(path, operation, io::Error::other("disk unavailable"));
        let StoreError::Io {
            path: actual_path,
            msg,
        } = error
        else {
            panic!("CAS write errors must retain the shared StoreError::Io category");
        };

        assert_eq!(actual_path, path);
        assert_eq!(msg, expected_message);
    }
}
