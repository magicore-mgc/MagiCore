//! Typed bounded-response and allocation failures.
//! Lỗi typed của response có giới hạn và cấp phát buffer.

use std::fmt;

#[derive(Debug)]
pub(super) enum BoundedResponseError {
    ExceedsByteLimit,
    AllocationFailed(std::collections::TryReserveError),
}

impl fmt::Display for BoundedResponseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExceedsByteLimit => formatter.write_str("HTTP response exceeds byte limit"),
            Self::AllocationFailed(_) => {
                formatter.write_str("cannot reserve bounded HTTP response body")
            }
        }
    }
}

impl std::error::Error for BoundedResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ExceedsByteLimit => None,
            Self::AllocationFailed(source) => Some(source),
        }
    }
}
