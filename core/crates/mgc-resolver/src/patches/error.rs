//! Coordinator errors for file-level patch sequencing.
//! Lỗi điều phối cho trình tự xử lý patch theo file.

#[derive(Debug, thiserror::Error)]
pub(super) enum PatchCoordinatorError {
    #[error("patch file mismatch: {old} vs {new}")]
    FileMismatch { old: String, new: String },
    #[error("target file not found in vstore: {0}")]
    TargetNotFound(String),
    #[error("expected unified-diff hunk header, found: {0}")]
    ExpectedHunkHeader(String),
    #[error("patch stream ended after a peeked hunk header")]
    HunkStreamEnded,
}
