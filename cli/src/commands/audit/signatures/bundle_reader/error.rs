//! Typed response-read errors with stable CLI mapping and retained source context.
//! Lỗi đọc response có kiểu, giữ source và ánh xạ CLI ổn định.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::audit::signatures) enum BundleReadError {
    #[error("trusted signatures response exceeds the configured size limit")]
    TooLarge,
    #[error("registry returned an invalid trusted signatures response")]
    Invalid(#[source] anyhow::Error),
}

impl BundleReadError {
    /// Keep the existing public diagnostic while retaining an internal cause.
    /// Giữ diagnostic công khai nhưng bảo toàn nguyên nhân nội bộ.
    pub fn into_cli(self) -> anyhow::Error {
        match self {
            Self::TooLarge => crate::error::trusted_signatures_response_too_large(),
            Self::Invalid(source) => {
                source.context(crate::error::trusted_signatures_response_invalid())
            }
        }
    }
}
