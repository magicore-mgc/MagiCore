//! Unit test for the local semaphore failure mapping.
//! Unit test cho mapping lỗi semaphore nội bộ.

use super::super::error::GraphResolutionError;

#[test]
fn semaphore_error_keeps_the_existing_message() {
    assert_eq!(
        GraphResolutionError::SemaphoreClosed.to_string(),
        "registry graph resolver semaphore closed"
    );
}
