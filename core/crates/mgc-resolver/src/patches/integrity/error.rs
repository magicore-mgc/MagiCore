//! Errors owned by patch-byte integrity verification.
//! Lỗi do module xác minh tính toàn vẹn byte patch sở hữu.

use std::io;

#[derive(Debug, thiserror::Error)]
pub(super) enum IntegrityError {
    #[error(transparent)]
    Read(#[from] io::Error),
}
