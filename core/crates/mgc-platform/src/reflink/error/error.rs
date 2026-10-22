//! Reflink error categories and their preserved source chain.
//! Phân loại lỗi reflink và giữ chuỗi nguyên nhân gốc.

use std::io;

/// Why a reflink clone could not be created.
/// Lý do không thể tạo reflink clone.
#[derive(Debug)]
pub enum ReflinkError {
    /// Platform/fs cannot reflink — caller must fall back (hardlink/copy).
    /// Nền tảng hoặc filesystem không hỗ trợ; caller cần fallback hardlink/copy.
    NotSupported(io::Error),
    /// Real failure — abort the operation, do not fall back.
    /// Lỗi thực thi — dừng thao tác, không chuyển sang fallback.
    Other(io::Error),
}

impl std::fmt::Display for ReflinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReflinkError::NotSupported(err) => {
                write!(f, "reflink not supported: {err}")
            }
            ReflinkError::Other(err) => write!(f, "reflink failed: {err}"),
        }
    }
}

impl std::error::Error for ReflinkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReflinkError::NotSupported(err) | ReflinkError::Other(err) => Some(err),
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
