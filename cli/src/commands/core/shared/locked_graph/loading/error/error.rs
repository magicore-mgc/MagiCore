//! Typed failures raised while loading a project lock graph.
//! Lỗi có kiểu phát sinh khi nạp lock graph của project.

#[cfg(test)]
use std::io::ErrorKind;

#[derive(Debug, thiserror::Error)]
pub(crate) enum LockedGraphLoadingError {
    #[error("lockfile '{path}' must be a regular non-symlink file")]
    UnsafeLockfilePath { path: String },
    #[error("{0}")]
    Lockfile(#[source] mgc_lockfile::LockfileError),
    #[error("{0}")]
    MetadataRead(#[source] std::io::Error),
    #[error("mgc.lock is not valid UTF-8: {source}")]
    InvalidUtf8 {
        #[source]
        source: std::string::FromUtf8Error,
    },
    #[error("cannot determine lock owner core: {source}")]
    ReadCoreMarker {
        #[source]
        source: anyhow::Error,
    },
    #[error("cannot read project core owner: {source}")]
    ReadProjectConfig {
        #[source]
        source: anyhow::Error,
    },
    #[error("refusing to read mgc.lock for unknown core owner '{core}'")]
    UnknownCoreOwner { core: String },
    #[error("{0}")]
    LegacyGraphConversion(#[source] mgc_types::MgError),
    #[error("{0}")]
    V4GraphConversion(#[source] mgc_lockfile::LockfileError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum LockedGraphRecovery {
    RetryRead,
    RepairLockfile,
    CorrectProjectConfiguration,
    CorrectFileAccess,
}

impl LockedGraphLoadingError {
    #[cfg(test)]
    pub(super) fn recovery(&self) -> LockedGraphRecovery {
        match self {
            Self::MetadataRead(source) if is_transient_io(source.kind()) => {
                LockedGraphRecovery::RetryRead
            }
            Self::Lockfile(mgc_lockfile::LockfileError::IoError(source))
                if is_transient_io(source.kind()) =>
            {
                LockedGraphRecovery::RetryRead
            }
            Self::Lockfile(mgc_lockfile::LockfileError::IoError(_)) | Self::MetadataRead(_) => {
                LockedGraphRecovery::CorrectFileAccess
            }
            Self::ReadCoreMarker { .. }
            | Self::ReadProjectConfig { .. }
            | Self::UnknownCoreOwner { .. } => LockedGraphRecovery::CorrectProjectConfiguration,
            Self::UnsafeLockfilePath { .. }
            | Self::Lockfile(_)
            | Self::InvalidUtf8 { .. }
            | Self::LegacyGraphConversion(_)
            | Self::V4GraphConversion(_) => LockedGraphRecovery::RepairLockfile,
        }
    }
}

#[cfg(test)]
fn is_transient_io(kind: ErrorKind) -> bool {
    matches!(
        kind,
        ErrorKind::Interrupted | ErrorKind::WouldBlock | ErrorKind::TimedOut
    )
}
