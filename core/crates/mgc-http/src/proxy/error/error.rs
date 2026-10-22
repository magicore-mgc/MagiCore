// Proxy construction errors — retain typed causes and stable display text.
// Lỗi khởi tạo proxy — giữ nguyên nguyên nhân có kiểu và thông điệp hiển thị.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Errors produced while parsing a target or constructing a reqwest proxy.
/// Lỗi khi phân tích URL đích hoặc tạo proxy reqwest.
#[derive(Debug)]
pub(in crate::proxy) enum ProxyError {
    InvalidTargetUrl(url::ParseError),
    InvalidProxyUrl(reqwest::Error),
}

impl Display for ProxyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTargetUrl(error) => Display::fmt(error, formatter),
            Self::InvalidProxyUrl(error) => write!(formatter, "invalid proxy url: {error}"),
        }
    }
}

impl Error for ProxyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTargetUrl(error) => Some(error),
            Self::InvalidProxyUrl(error) => Some(error),
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
