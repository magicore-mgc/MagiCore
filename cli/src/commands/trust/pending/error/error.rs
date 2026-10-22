//! Typed local failures for the trust-pending report.
//! Lỗi cục bộ có kiểu cho báo cáo trust-pending.

use std::error::Error as StdError;
use std::path::Path;

pub(in crate::commands::trust::pending) type PendingResult<T> =
    std::result::Result<T, TrustPendingError>;

/// Typed errors retain their original source and preserve the English CLI text.
/// Lỗi typed giữ source gốc và nguyên văn thông báo CLI tiếng Anh.
#[derive(Debug, thiserror::Error)]
pub(in crate::commands::trust::pending) enum TrustPendingError {
    #[error(transparent)]
    PendingJournal(#[from] anyhow::Error),
    #[error(
        "cannot scan trust-pending packages in '{root}' while project state is changing or unrecovered: {source}"
    )]
    LockFailed {
        root: String,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
    #[error("failed to load [scripts] trust policy: {source}")]
    ScriptsPolicyInvalid {
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
    #[error("failed to inspect lifecycle-script package data at '{path}': {source}")]
    ScanFailed {
        path: String,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
    #[error("invalid lifecycle-script package manifest '{path}': {reason}")]
    ManifestInvalid { path: String, reason: String },
    #[error("lifecycle-script package manifest '{path}' exceeds the {limit}-byte inspection limit")]
    ManifestTooLarge { path: String, limit: u64 },
}

impl TrustPendingError {
    pub(crate) fn lock_failed<E>(root: &Path, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::LockFailed {
            root: root.display().to_string(),
            source: Box::new(source),
        }
    }

    pub(crate) fn scripts_policy_failed<E>(source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::ScriptsPolicyInvalid {
            source: Box::new(source),
        }
    }

    pub(crate) fn scan_failed<E>(path: &Path, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::ScanFailed {
            path: path.display().to_string(),
            source: Box::new(source),
        }
    }

    pub(crate) fn manifest_invalid(path: &Path, reason: impl Into<String>) -> Self {
        Self::ManifestInvalid {
            path: path.display().to_string(),
            reason: reason.into(),
        }
    }

    pub(crate) fn manifest_too_large(path: &Path, limit: u64) -> Self {
        Self::ManifestTooLarge {
            path: path.display().to_string(),
            limit,
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
