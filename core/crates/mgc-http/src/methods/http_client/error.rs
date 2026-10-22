//! Typed client-construction failures with the reqwest source retained.
//! Lỗi typed khi tạo client, giữ nguyên source từ reqwest.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(super) enum HttpClientError {
    BuildClient(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for HttpClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuildClient(source) => write!(formatter, "build reqwest client: {source}"),
        }
    }
}

impl Error for HttpClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BuildClient(source) => Some(source.as_ref()),
        }
    }
}

impl From<reqwest::Error> for HttpClientError {
    fn from(error: reqwest::Error) -> Self {
        Self::BuildClient(Box::new(error))
    }
}
