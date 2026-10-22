//! Own typed Python manifest and lock-pin failures.
//! Sở hữu lỗi typed của manifest Python và pin trong lock.

use mgc_types::MgError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub(in crate::install::python_runtime::path_entries) enum PythonManifestGuardError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error(
        "pyproject.toml uses unsupported or externally-owned Python dependency metadata; refusing to run with ambient Python packages"
    )]
    UnsupportedMetadata,
    #[error("refusing symlinked pyproject.toml for Python runtime")]
    SymlinkedManifest,
    #[error("pyproject.toml for Python runtime is not a regular file")]
    NonRegularManifest,
    #[error("cannot inspect pyproject.toml for Python runtime: {source}")]
    ManifestInspection { source: io::Error },
    #[error(
        "Python dependency manifest '{manifest_name}' is not owned by the native MagiCore resolver; refusing to run with ambient Python packages"
    )]
    ExternalManifest { manifest_name: String },
    #[error(
        "Python dependency '{dependency}' declared in pyproject.toml is absent from the {owner_core}-owned mgc.lock; run `mgc install --core {owner_core}`"
    )]
    DependencyAbsentFromLock {
        dependency: String,
        owner_core: String,
    },
    #[error(
        "Python dependency '{dependency}' has an uncheckable locked version '{locked_version}': {source}"
    )]
    UncheckableLockedVersion {
        dependency: String,
        locked_version: String,
        #[source]
        source: Box<MgError>,
    },
    #[error(
        "Python dependency '{dependency}' requires '{required}' but mgc.lock pins '{locked_version}'; run `mgc install --core {owner_core}`"
    )]
    LockedVersionMismatch {
        dependency: String,
        required: String,
        locked_version: String,
        owner_core: String,
    },
}

impl From<PythonManifestGuardError> for MgError {
    fn from(error: PythonManifestGuardError) -> Self {
        match error {
            PythonManifestGuardError::Shared(error) => error,
            error => Self::Other(error.to_string()),
        }
    }
}
