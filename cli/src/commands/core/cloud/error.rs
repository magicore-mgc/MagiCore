//! Typed errors for Cloud adapter selection.
//! Lỗi có kiểu khi chọn adapter Cloud.

use std::error::Error as StdError;
use std::fmt;

/// Boundary labels Cloud adapter binding failures and retains their causes.
/// Boundary phân loại lỗi gắn adapter Cloud và giữ nguyên nguyên nhân.
#[derive(Debug)]
pub(super) enum CloudAdapterError {
    ProjectIdentity { source: anyhow::Error },
    ProjectContext { source: anyhow::Error },
}

impl CloudAdapterError {
    pub(super) fn project_identity(source: anyhow::Error) -> Self {
        Self::ProjectIdentity { source }
    }

    pub(super) fn project_context(source: anyhow::Error) -> Self {
        Self::ProjectContext { source }
    }

    fn source_error(&self) -> &anyhow::Error {
        match self {
            Self::ProjectIdentity { source } | Self::ProjectContext { source } => source,
        }
    }
}

impl fmt::Display for CloudAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.source_error(), formatter)
    }
}

impl StdError for CloudAdapterError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source_error().as_ref())
    }
}
