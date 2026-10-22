//! Typed errors for Lib adapter selection.
//! Lỗi có kiểu khi chọn adapter Lib.

use std::error::Error as StdError;
use std::fmt;

/// Boundary labels the Lib adapter construction failure and keeps its source.
/// Boundary phân loại lỗi dựng adapter Lib và giữ nguyên source.
#[derive(Debug)]
pub(super) enum LibAdapterError {
    ProjectIdentity { source: anyhow::Error },
    ProjectContext { source: anyhow::Error },
    RegistryAdapter { source: anyhow::Error },
}

impl LibAdapterError {
    pub(super) fn project_identity(source: anyhow::Error) -> Self {
        Self::ProjectIdentity { source }
    }

    pub(super) fn project_context(source: anyhow::Error) -> Self {
        Self::ProjectContext { source }
    }

    pub(super) fn registry_adapter(source: anyhow::Error) -> Self {
        Self::RegistryAdapter { source }
    }

    fn source_error(&self) -> &anyhow::Error {
        match self {
            Self::ProjectIdentity { source }
            | Self::ProjectContext { source }
            | Self::RegistryAdapter { source } => source,
        }
    }
}

impl fmt::Display for LibAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.source_error(), formatter)
    }
}

impl StdError for LibAdapterError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source_error().as_ref())
    }
}
