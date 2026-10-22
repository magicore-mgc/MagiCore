// Start-upload errors — failures owned by session creation.
// Lỗi khởi tạo upload — lỗi thuộc thao tác tạo session.

use reqwest::StatusCode;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(super) enum StartUploadError {
    Failed(StatusCode),
    MissingLocation,
    InvalidLocation(String),
}

impl Display for StartUploadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed(status) => write!(formatter, "start upload failed: {status}"),
            Self::MissingLocation => formatter.write_str("missing Location header"),
            Self::InvalidLocation(location) => write!(formatter, "invalid Location: {location}"),
        }
    }
}

impl Error for StartUploadError {}
