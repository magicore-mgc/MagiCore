//! Typed failures for App lockfile mutation / Lỗi có kiểu cho thao tác lockfile App.

use mgc_types::MgError;
use std::error::Error;

type ErrorSource = Box<dyn Error + Send + Sync + 'static>;

/// Error contract local to the App lock writer.
/// Hợp đồng lỗi cục bộ của bộ ghi lock App.
#[derive(Debug, thiserror::Error)]
pub(in crate::install::lockfile) enum AppLockfileError {
    /// A local validation or policy refusal without an underlying source.
    /// Từ chối kiểm tra/chính sách cục bộ, không có source bên dưới.
    #[error("{message}")]
    Message {
        /// Stable internal diagnostic code.
        /// Mã diagnostic nội bộ ổn định.
        code: &'static str,
        /// Operation which failed.
        /// Operation gặp lỗi.
        operation: &'static str,
        /// Lock mutation failures are not retried automatically.
        /// Lỗi mutation lock không được tự động retry.
        retryable: bool,
        /// Existing user-facing English diagnostic, kept stable.
        /// Diagnostic tiếng Anh hiện có, được giữ ổn định.
        message: String,
    },
    /// A failure with a retained source chain.
    /// Lỗi có source chain được giữ lại.
    #[error("{message}")]
    Cause {
        /// Stable internal diagnostic code.
        /// Mã diagnostic nội bộ ổn định.
        code: &'static str,
        /// Operation which failed.
        /// Operation gặp lỗi.
        operation: &'static str,
        /// Lock mutation failures are not retried automatically.
        /// Lỗi mutation lock không được tự động retry.
        retryable: bool,
        /// Existing user-facing English diagnostic, kept stable.
        /// Diagnostic tiếng Anh hiện có, được giữ ổn định.
        message: String,
        /// Underlying cause retained until shared-error conversion.
        /// Nguyên nhân bên dưới được giữ tới bước ánh xạ lỗi chung.
        #[source]
        source: ErrorSource,
    },
}

impl AppLockfileError {
    pub(in crate::install::lockfile) fn message(
        code: &'static str,
        operation: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self::Message {
            code,
            operation,
            retryable: false,
            message: message.into(),
        }
    }

    pub(in crate::install::lockfile) fn caused<E>(
        code: &'static str,
        operation: &'static str,
        message: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync + 'static>>,
    {
        Self::Cause {
            code,
            operation,
            retryable: false,
            message: message.into(),
            source: source.into(),
        }
    }

    #[cfg(test)]
    pub(in crate::install::lockfile) fn code(&self) -> &'static str {
        match self {
            Self::Message { code, .. } | Self::Cause { code, .. } => code,
        }
    }

    #[cfg(test)]
    pub(in crate::install::lockfile) fn operation(&self) -> &'static str {
        match self {
            Self::Message { operation, .. } | Self::Cause { operation, .. } => operation,
        }
    }

    #[cfg(test)]
    pub(in crate::install::lockfile) fn is_retryable(&self) -> bool {
        match self {
            Self::Message { retryable, .. } | Self::Cause { retryable, .. } => *retryable,
        }
    }
}

impl From<AppLockfileError> for MgError {
    fn from(error: AppLockfileError) -> Self {
        Self::Other(error.to_string())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
