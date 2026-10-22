//! Maps local CAS write operations into the shared store I/O error.
//! Ánh xạ thao tác ghi CAS cục bộ sang lỗi I/O dùng chung của store.

use std::io;
use std::path::Path;

use crate::cas::store::StoreError;

#[derive(Clone, Copy)]
pub(super) enum CasWriteOperation {
    Read,
    Write,
    Flush,
    FlushAndSync,
    SetPermissions,
    ParentDirectorySync,
}

impl CasWriteOperation {
    fn label(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Flush => "flush",
            Self::FlushAndSync => "flush/fsync",
            Self::SetPermissions => "set permissions",
            Self::ParentDirectorySync => "fsync parent directory",
        }
    }
}

pub(super) fn map_io_error(
    path: &Path,
    operation: CasWriteOperation,
    source: io::Error,
) -> StoreError {
    StoreError::Io {
        path: path.to_path_buf(),
        msg: format!("{} failed: {source}", operation.label()),
    }
}

#[cfg(test)]
#[path = "error/test/mod.rs"]
mod test;
