//! Own typed Python lock-owner failures.
//! Sở hữu lỗi typed khi đọc lock Python.

use mgc_types::MgError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub(in crate::install::python_runtime::path_entries) enum PythonLockOwnerError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("{source}")]
    SchemaDetection {
        #[source]
        source: anyhow::Error,
    },
    #[error("cannot inspect Python lockfile '{path}': {source}")]
    LockfileInspection { path: String, source: io::Error },
    #[error("cannot safely read mgc.lock for Python runtime: {source}")]
    V3LockSafety {
        #[source]
        source: mgc_lockfile::LockfileError,
    },
    #[error("cannot safely read mgc.lock for Python runtime: {source}")]
    V4LockSafety {
        #[source]
        source: MgError,
    },
    #[error(
        "Python project declares dependencies in pyproject.toml but has no mgc.lock; run `mgc install --core {owner_core}` before launching Python"
    )]
    DeclaredDependenciesWithoutLock { owner_core: String },
}

impl From<PythonLockOwnerError> for MgError {
    fn from(error: PythonLockOwnerError) -> Self {
        match error {
            PythonLockOwnerError::Shared(error) => error,
            error => Self::Other(error.to_string()),
        }
    }
}
