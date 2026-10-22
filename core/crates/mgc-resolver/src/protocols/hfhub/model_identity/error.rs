//! Typed local errors for model identity validation.
//! Lỗi cục bộ có type cho kiểm tra định danh model.

use std::error::Error;
use std::fmt;

use mgc_types::MgError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::protocols::hfhub) enum ModelIdentityError {
    InvalidModelId(String),
    InvalidRevision(String),
}

impl fmt::Display for ModelIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModelId(model) => {
                write!(
                    formatter,
                    "invalid model id '{model}' — expected org/name (fail-closed)"
                )
            }
            Self::InvalidRevision(revision) => write!(
                formatter,
                concat!(
                    "revision '{revision}' is not an exact 40-hex commit SHA — ",
                    "branch/tag names are mutable refs and cannot lock (fail-closed)"
                ),
                revision = revision
            ),
        }
    }
}

impl Error for ModelIdentityError {}

impl From<ModelIdentityError> for MgError {
    fn from(error: ModelIdentityError) -> Self {
        MgError::Other(error.to_string())
    }
}
