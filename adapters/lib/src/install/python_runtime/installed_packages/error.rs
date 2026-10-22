//! Own typed Python installed-package listing failures.
//! Sở hữu lỗi typed khi liệt kê package Python đã cài.

use mgc_types::MgError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub(super) enum PythonInstalledPackagesError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("cannot inspect mgc.lock for Python package listing: {source}")]
    LockfileInspection { source: io::Error },
    #[error("Python lock/materialization count changed while listing; retry the operation")]
    LockMaterializationChanged,
}

impl From<PythonInstalledPackagesError> for MgError {
    fn from(error: PythonInstalledPackagesError) -> Self {
        match error {
            PythonInstalledPackagesError::Shared(error) => error,
            error => Self::Other(error.to_string()),
        }
    }
}
