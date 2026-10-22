//! Owns best-effort quarantine evidence for corrupted blobs.
//! Quản lý cách ly best-effort và bằng chứng của blob hỏng.

mod error;
mod quarantine;
pub(super) use self::error::QuarantineError;
pub(super) use self::quarantine::quarantine_blob;
