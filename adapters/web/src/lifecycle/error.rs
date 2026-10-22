//! Typed lifecycle failures with preserved causes and recovery classification.
//! Lỗi lifecycle có kiểu, giữ nguyên nguyên nhân và phân loại phục hồi.

use mgc_types::error::MgError;

pub(crate) type LifecycleResult<T> = Result<T, LifecycleError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum LifecycleFailureClass {
    Manifest,
    Policy,
    Execution,
    ProjectIdentity,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum LifecycleError {
    #[error("failed to safely read package.json for lifecycle '{path}': {source}")]
    ManifestRead {
        path: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("failed to parse package.json for lifecycle '{path}': {source}")]
    ManifestParse {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("cannot validate project core identity before lifecycle scripts: {source}")]
    IdentityPreflight {
        #[source]
        source: anyhow::Error,
    },
    #[error("cannot snapshot project identity before lifecycle scripts: {source}")]
    IdentitySnapshot {
        #[source]
        source: anyhow::Error,
    },
    #[error(
        "project core identity could not be verified during lifecycle scripts and was restored: {source}"
    )]
    IdentityVerificationRestored {
        #[source]
        source: anyhow::Error,
    },
    #[error(
        "project core identity could not be verified during lifecycle scripts: {source}; \
         restoration also failed: {restore}"
    )]
    IdentityVerificationRestoreFailed {
        #[source]
        source: anyhow::Error,
        restore: anyhow::Error,
    },
    #[error(
        "lifecycle scripts modified project core identity files; install aborted; \
         original identity files were restored"
    )]
    IdentityModifiedRestored,
    #[error(
        "lifecycle scripts modified project core identity files; install aborted; \
         restoration also failed: {restore}"
    )]
    IdentityModifiedRestoreFailed {
        #[source]
        restore: anyhow::Error,
    },
    #[error(
        "project core identity became invalid during lifecycle scripts and was restored: {source}"
    )]
    IdentityInvalidRestored {
        #[source]
        source: anyhow::Error,
    },
    #[error(
        "project core identity became invalid during lifecycle scripts: {source}; \
         restoration also failed: {restore}"
    )]
    IdentityInvalidRestoreFailed {
        #[source]
        source: anyhow::Error,
        restore: anyhow::Error,
    },
    #[error(
        "lifecycle scripts changed project core identity from '{expected}' to '{actual}'; \
         install aborted; original identity files were restored"
    )]
    IdentityChangedRestored { expected: String, actual: String },
    #[error(
        "lifecycle scripts changed project core identity from '{expected}' to '{actual}'; \
         install aborted; restoration also failed: {restore}"
    )]
    IdentityChangedRestoreFailed {
        expected: String,
        actual: String,
        #[source]
        restore: anyhow::Error,
    },
    #[error(
        "lifecycle script in '{path}' delegates to '{tool}'; core-web refuses package-manager \
         wrappers inside lifecycle execution"
    )]
    ExternalPackageManager { path: String, tool: String },
    #[error("unsupported lifecycle script '{name}': {source}")]
    UnsupportedScript {
        name: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("lifecycle script '{name}' failed: {source}")]
    ScriptFailed {
        name: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("failed to build lifecycle PATH: {0}")]
    PathBuild(#[source] std::env::JoinPathsError),
    #[error("lifecycle injected failure for '{package}' (MGC_LIFECYCLE_FAIL_PACKAGES)")]
    InjectedFailure { package: String },
}

impl LifecycleError {
    #[cfg(test)]
    pub(crate) fn class(&self) -> LifecycleFailureClass {
        match self {
            Self::ManifestRead { .. } | Self::ManifestParse { .. } => {
                LifecycleFailureClass::Manifest
            }
            Self::ExternalPackageManager { .. }
            | Self::UnsupportedScript { .. }
            | Self::InjectedFailure { .. } => LifecycleFailureClass::Policy,
            Self::IdentityPreflight { .. }
            | Self::IdentitySnapshot { .. }
            | Self::IdentityVerificationRestored { .. }
            | Self::IdentityVerificationRestoreFailed { .. }
            | Self::IdentityModifiedRestored
            | Self::IdentityModifiedRestoreFailed { .. }
            | Self::IdentityInvalidRestored { .. }
            | Self::IdentityInvalidRestoreFailed { .. }
            | Self::IdentityChangedRestored { .. }
            | Self::IdentityChangedRestoreFailed { .. } => LifecycleFailureClass::ProjectIdentity,
            Self::ScriptFailed { .. } | Self::PathBuild(_) => LifecycleFailureClass::Execution,
        }
    }
}

impl From<LifecycleError> for MgError {
    fn from(error: LifecycleError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
