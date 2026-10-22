//! Typed cache-presence errors — giữ source I/O khi kiểm tra model path.

use mgc_types::MgError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to inspect AI model cache path")]
pub(in crate::cache::is_cached) struct IsCachedError(#[source] io::Error);

impl IsCachedError {
    pub(in crate::cache::is_cached) fn io(source: io::Error) -> Self {
        Self(source)
    }
}

impl From<IsCachedError> for MgError {
    fn from(error: IsCachedError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
