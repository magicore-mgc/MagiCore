//! Typed failures for patch-relative path and vstore containment checks.
//! Lỗi có kiểu cho đường dẫn patch và kiểm tra nằm trong kho package.

use std::io;

#[derive(Debug, thiserror::Error)]
pub(crate) enum PathValidationError {
    #[error("unsafe patch path: expected a non-empty slash-separated relative path")]
    InvalidRelativeForm,
    #[error("unsafe patch path: path traversal or absolute paths are forbidden: {0}")]
    Traversal(String),
    #[error(transparent)]
    Canonicalize(#[from] io::Error),
    #[error("patch target escapes package store: {0}")]
    EscapesStore(String),
    #[error("patch target is not a regular file: {0}")]
    NotRegularFile(String),
}
