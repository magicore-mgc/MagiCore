// Finalize errors — failures owned by the final PUT.
// Lỗi finalize — lỗi thuộc request PUT cuối cùng.

use reqwest::StatusCode;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(super) enum FinalizeUploadError {
    Failed(StatusCode),
    MissingLocation,
}

impl Display for FinalizeUploadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed(status) => write!(formatter, "finalize upload failed: {status}"),
            Self::MissingLocation => formatter.write_str("missing Location after finalize"),
        }
    }
}

impl Error for FinalizeUploadError {}
