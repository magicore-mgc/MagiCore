// File-upload errors — preserve the existing open-file context.
// Lỗi upload file — giữ nguyên context mở file hiện có.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(super) struct OpenUploadFileError(std::io::Error);

impl Display for OpenUploadFileError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "open file: {}", self.0)
    }
}

impl Error for OpenUploadFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl From<std::io::Error> for OpenUploadFileError {
    fn from(error: std::io::Error) -> Self {
        Self(error)
    }
}
