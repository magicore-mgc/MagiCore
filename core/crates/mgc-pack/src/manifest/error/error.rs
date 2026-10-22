//! Typed failures for manifest validation with preserved user-facing text.
//! Lỗi typed cho validation manifest, giữ nguyên nội dung hiển thị.

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ManifestSanitizeError {
    NotJsonObject,
    PrivatePackage,
    InvalidPrivateType,
    MissingName,
    MissingVersion,
}

impl Display for ManifestSanitizeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::NotJsonObject => "manifest is not a JSON object",
            Self::PrivatePackage => {
                "package.json has private: true — cannot publish (remove private: true to publish)"
            }
            Self::InvalidPrivateType => "package.json private must be a boolean",
            Self::MissingName => "package.json missing name",
            Self::MissingVersion => "package.json missing version",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ManifestSanitizeError {}
