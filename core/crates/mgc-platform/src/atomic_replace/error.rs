//! Typed replacement failures with the original operating-system source.
//! Lỗi thay file typed, giữ source gốc từ hệ điều hành.

use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
/// Atomic replacement failure with the source paths and OS error.
/// Lỗi thay file atomic kèm pathname và lỗi hệ điều hành gốc.
pub enum AtomicReplaceError {
    #[error("atomic replacement failed from {staging} to {destination}: {source}")]
    /// The selected native replacement operation failed.
    /// Thao tác thay file native được chọn đã thất bại.
    Replace {
        staging: PathBuf,
        destination: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl AtomicReplaceError {
    /// Returns the original I/O error without discarding this typed context.
    /// Trả lỗi I/O gốc mà không làm mất context typed của owner.
    pub fn io_source(&self) -> &io::Error {
        match self {
            Self::Replace { source, .. } => source,
        }
    }
}
