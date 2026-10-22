//! Typed manifest failures with stable display and retained causes.
//! Lỗi manifest typed, giữ display ổn định và nguyên nhân.

use mgc_platform::OpenRegularFileError;
use mgc_types::MgError;
use std::path::PathBuf;
use thiserror::Error;

/// Failures owned by the shared JS manifest subsystem.
/// Các lỗi do phân hệ manifest JS dùng chung sở hữu.
#[derive(Debug, Error)]
pub(in crate::manifest) enum ManifestError {
    #[error(
        "No package.json in '{project_root}'. Run 'mgc init --template web' first.",
        project_root = .project_root.display()
    )]
    MissingPackageJson { project_root: PathBuf },
    #[error("package.json is too large ({size} bytes, max {max})")]
    ManifestTooLarge { size: u64, max: u64 },
    #[error("invalid version '{version}' in package.json")]
    InvalidVersion { version: String },
    #[error("refusing to read unsafe package.json: {source}")]
    OpenPackageJson {
        #[source]
        source: OpenRegularFileError,
    },
    #[error(
        "refusing to atomically write non-regular or linked path: {path}",
        path = .path.display()
    )]
    UnsafeAtomicWriteDestination { path: PathBuf },
    #[error("inspect atomic-write destination {path}: {source}", path = .path.display())]
    InspectAtomicWriteDestination {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid write path: {path}", path = .path.display())]
    InvalidAtomicWritePath { path: PathBuf },
    #[error("create atomic staging file: {source}")]
    CreateAtomicStagingFile {
        #[source]
        source: std::io::Error,
    },
    #[error("sync atomic staging file: {source}")]
    SyncAtomicStagingFile {
        #[source]
        source: std::io::Error,
    },
    #[error(
        "atomic-write destination appeared during publication: {path}",
        path = .path.display()
    )]
    AtomicWriteDestinationAppeared { path: PathBuf },
    #[error("recheck atomic-write destination {path}: {source}", path = .path.display())]
    RecheckAtomicWriteDestination {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("publish atomic file: {source}")]
    PublishAtomicFile {
        #[source]
        source: MgError,
    },
    #[cfg(unix)]
    #[error("sync atomic-write directory: {source}")]
    SyncAtomicWriteDirectory {
        #[source]
        source: std::io::Error,
    },
    #[error(
        "refusing to update non-regular or linked path: {path}",
        path = .path.display()
    )]
    UnsafeUpdateDestination { path: PathBuf },
    #[error("inspect update destination {path}: {source}", path = .path.display())]
    InspectUpdateDestination {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("inspect package.json: {source}")]
    InspectPackageJson {
        #[source]
        source: std::io::Error,
    },
    #[error("read existing package.json: {source}")]
    ExistingPackageRead {
        #[source]
        source: anyhow::Error,
    },
    #[error("workspace dependency '{name}' cannot be silently replaced by a registry pin")]
    WorkspaceDependencyReplacement { name: String },
}

impl From<ManifestError> for MgError {
    fn from(error: ManifestError) -> Self {
        match error {
            ManifestError::WorkspaceDependencyReplacement { name } => MgError::Unsupported {
                core: "web",
                capability: "replace workspace dependency source",
                guidance: format!(
                    "dependency '{name}' is currently workspace-owned; MagiCore refuses to silently replace its source with a registry pin"
                ),
            },
            ManifestError::OpenPackageJson {
                source: OpenRegularFileError::Io(source),
            } => MgError::Io(source),
            error => MgError::Anyhow(anyhow::Error::new(error)),
        }
    }
}
