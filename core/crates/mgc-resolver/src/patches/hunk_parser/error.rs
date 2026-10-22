//! Typed errors for unified-diff header, range and operation parsing.
//! Lỗi có kiểu cho header, phạm vi và thao tác unified diff.

use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub(in crate::patches) enum HunkParseError {
    #[error("expected {0} header")]
    ExpectedHeader(String),
    #[error("expected line starting with {0}")]
    ExpectedPrefix(String),
    #[error("invalid {0} header")]
    InvalidHeader(String),
    #[error("invalid hunk header: {0}")]
    InvalidHunkHeader(String),
    #[error("patch stream ended after a peeked patch line")]
    PatchLineStreamEnded,
    #[error("invalid hunk operation: {0:?}")]
    InvalidOperation(char),
    #[error("invalid UTF-8 boundary in patch hunk")]
    InvalidUtf8Boundary,
    #[error(transparent)]
    InvalidPath(#[from] crate::patches::path_validation::PathValidationError),
    #[error(transparent)]
    InvalidInteger(#[from] ParseIntError),
}
