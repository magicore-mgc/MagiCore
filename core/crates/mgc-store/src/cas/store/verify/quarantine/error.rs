//! Typed failures while preserving CAS mismatch evidence.
//! Lỗi typed khi lưu bằng chứng CAS không khớp.

use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::cas::store::verify) enum QuarantineError {
    #[error("{operation} failed at {path}: {source}")]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("quarantine copy digest mismatch: expected {expected}, got {actual}")]
    DigestMismatch { expected: String, actual: String },
}
