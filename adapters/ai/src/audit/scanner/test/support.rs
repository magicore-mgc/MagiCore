//! Shared temporary-directory helper for scanner tests.
//! Helper thư mục tạm dùng chung cho test scanner.

use tempfile::TempDir;

pub(super) fn tmp() -> TempDir {
    TempDir::new().unwrap()
}
