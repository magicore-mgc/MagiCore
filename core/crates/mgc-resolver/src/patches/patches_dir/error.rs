//! Errors owned by global patch-directory discovery.
//! Lỗi do việc tìm thư mục patch toàn cục sở hữu.

use std::io;

#[derive(Debug, thiserror::Error)]
pub(super) enum PatchesDirectoryError {
    #[error(transparent)]
    GlobalPaths(#[from] io::Error),
}
