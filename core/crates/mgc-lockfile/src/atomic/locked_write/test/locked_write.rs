//! Verifies local source retention and stable public write-error mapping.
//! Kiểm tra giữ source cục bộ và ánh xạ lỗi công khai ổn định.

use super::super::LockedWriteError;

#[test]
fn write_failure_keeps_source_and_maps_to_legacy_category_and_message() {
    let error = LockedWriteError::WriteTemp {
        path: "project/mgc.lock.tmp.1.x".to_string(),
        source: std::io::Error::other("disk full"),
    };
    assert_eq!(
        std::error::Error::source(&error)
            .expect("test operation must succeed")
            .to_string(),
        "disk full"
    );
    let mapped: crate::LockfileError = error.into();
    assert!(matches!(
        mapped,
        crate::LockfileError::WriteFailed(message)
            if message == "cannot write lock temp 'project/mgc.lock.tmp.1.x': disk full"
    ));
}
