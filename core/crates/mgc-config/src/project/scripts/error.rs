//! Typed errors produced by committed script-policy loading.
//! Lỗi có kiểu khi đọc policy script đã commit.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// Failure while reading or decoding the committed `[scripts]` table.
/// Lỗi khi đọc hoặc giải mã bảng `[scripts]` đã commit.
#[derive(Debug)]
pub enum ScriptsPolicyError {
    /// Project config could not be safely read — không thể đọc an toàn cấu hình project.
    ReadProjectConfig {
        path: PathBuf,
        source: anyhow::Error,
    },
    /// The project file is not valid TOML — file project không phải TOML hợp lệ.
    InvalidToml {
        path: PathBuf,
        source: toml::de::Error,
    },
    /// The `[scripts]` value could not be converted to the internal value format.
    /// Không thể chuyển giá trị `[scripts]` sang định dạng nội bộ.
    EncodeScriptsTable {
        path: PathBuf,
        source: serde_json::Error,
    },
    /// The `[scripts]` value does not match the committed policy schema.
    /// Giá trị `[scripts]` không khớp schema policy đã commit.
    DecodeScriptsTable {
        path: PathBuf,
        source: serde_json::Error,
    },
}

impl Display for ScriptsPolicyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadProjectConfig { path, source } => {
                write!(formatter, "cannot read {}: {source}", path.display())
            }
            Self::InvalidToml { path, source } => {
                write!(formatter, "invalid TOML in {}: {source}", path.display())
            }
            Self::EncodeScriptsTable { path, source }
            | Self::DecodeScriptsTable { path, source } => {
                write!(
                    formatter,
                    "invalid [scripts] table in {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl Error for ScriptsPolicyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadProjectConfig { source, .. } => Some(source.as_ref()),
            Self::InvalidToml { source, .. } => Some(source),
            Self::EncodeScriptsTable { source, .. } | Self::DecodeScriptsTable { source, .. } => {
                Some(source)
            }
        }
    }
}
