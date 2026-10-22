//! Typed failures owned by App multi-platform build selection.
//! Lỗi typed thuộc chọn platform cho build App đa nền tảng.

use std::fmt;

#[derive(Debug)]
pub(super) enum AppMultiBuildError {
    InvalidPlatforms { reason: String },
    Incomplete { skipped: Vec<String> },
}

impl fmt::Display for AppMultiBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPlatforms { reason } => {
                write!(
                    formatter,
                    "invalid App build platforms configuration: {reason}"
                )
            }
            Self::Incomplete { skipped } => write!(
                formatter,
                "multi-platform build incomplete; selected target(s) not built: {}",
                skipped.join(", ")
            ),
        }
    }
}

impl std::error::Error for AppMultiBuildError {}
