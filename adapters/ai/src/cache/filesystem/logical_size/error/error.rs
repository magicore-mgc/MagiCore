//! Typed cache measurement errors — giữ operation và lỗi I/O gốc.

use mgc_types::MgError;
use std::io;
use thiserror::Error;

pub(in crate::cache) type LogicalSizeResult<T> = Result<T, LogicalSizeError>;

#[derive(Debug, Error)]
pub(in crate::cache) enum LogicalSizeError {
    #[error("failed to {operation} AI cache data")]
    Io {
        operation: &'static str,
        #[source]
        source: io::Error,
    },
}

impl LogicalSizeError {
    pub(in crate::cache) fn io(operation: &'static str, source: io::Error) -> Self {
        Self::Io { operation, source }
    }
}

impl From<LogicalSizeError> for MgError {
    fn from(error: LogicalSizeError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
