// Chunk-upload errors — overflow and unsuccessful server status.
// Lỗi upload chunk — tràn offset và status server không thành công.

use reqwest::StatusCode;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(super) enum ChunkUploadError {
    OffsetOverflow,
    Failed(StatusCode),
}

impl Display for ChunkUploadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OffsetOverflow => formatter.write_str("OCI blob upload offset overflow"),
            Self::Failed(status) => write!(formatter, "upload chunk failed: {status}"),
        }
    }
}

impl Error for ChunkUploadError {}
