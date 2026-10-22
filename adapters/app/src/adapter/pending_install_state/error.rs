//! Typed errors for pending install state access.
//! Lỗi có kiểu cho truy cập state install đang chờ.

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(super) enum PendingInstallStateError {
    LockPoisoned,
}

impl Display for PendingInstallStateError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockPoisoned => formatter.write_str(
                "app pending lock poisoned — refusing rather than operating on uncertain state",
            ),
        }
    }
}

impl std::error::Error for PendingInstallStateError {}

impl From<PendingInstallStateError> for mgc_types::MgError {
    fn from(error: PendingInstallStateError) -> Self {
        Self::Other(error.to_string())
    }
}
