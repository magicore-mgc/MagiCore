//! Watcher error display/source preservation test.
//! Test giữ nguyên hiển thị/source của lỗi watcher.

use std::error::Error as StdError;

use super::HmrWatchError;

#[test]
fn preserves_notify_message_and_source() {
    let original = notify::Error::generic("watch unavailable");
    let error = HmrWatchError::from(original);

    assert_eq!(error.to_string(), "watch unavailable");
    assert!(StdError::source(&error).is_some());
}
