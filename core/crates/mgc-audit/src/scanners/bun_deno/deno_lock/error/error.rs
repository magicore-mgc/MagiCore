//! Typed Deno lock parse failures — lỗi parse lock Deno có kiểu riêng.
//! Preserve legacy display text and the underlying serde error source.
//! Giữ nội dung hiển thị cũ và source lỗi serde bên dưới.

use mgc_types::MgError;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(in crate::scanners::bun_deno::deno_lock) enum DenoLockError {
    InvalidJson(serde_json::Error),
    NpmPinWithoutVersion(String),
}

impl Display for DenoLockError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(source) => write!(formatter, "invalid deno.lock: {source}"),
            Self::NpmPinWithoutVersion(pin) => {
                write!(formatter, "deno.lock npm pin '{pin}' must be name@version")
            }
        }
    }
}

impl Error for DenoLockError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidJson(source) => Some(source),
            Self::NpmPinWithoutVersion(_) => None,
        }
    }
}

impl From<DenoLockError> for MgError {
    fn from(error: DenoLockError) -> Self {
        Self::Other(error.to_string())
    }
}
