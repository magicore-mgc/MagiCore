//! Typed failures for registry request validation.
//! Lỗi có kiểu khi kiểm tra request registry.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum RequestValidationError {
    InvalidRequest { reason: String },
}

impl Display for RequestValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest { reason } => write!(
                formatter,
                "package runner: invalid registry package request: {reason}"
            ),
        }
    }
}

impl std::error::Error for RequestValidationError {}

pub fn registry_package_invalid_request(reason: &str) -> AnyhowError {
    AnyhowError::new(RequestValidationError::InvalidRequest {
        reason: reason.to_owned(),
    })
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
