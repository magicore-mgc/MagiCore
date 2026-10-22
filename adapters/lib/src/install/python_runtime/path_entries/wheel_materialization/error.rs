//! Own typed Python wheel and site-materialization failures.
//! Sở hữu lỗi typed của wheel Python và site đã materialize.

use mgc_types::MgError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub(in crate::install::python_runtime::path_entries) enum PythonWheelMaterializationError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("Python dependency '{name}'@{version} has no SHA-256 integrity in mgc.lock")]
    MissingIntegrity { name: String, version: String },
    #[error(
        "Python dependency '{name}'@{version} has a wheel that does not match this host/runtime; set MGC_PYTHON_VERSION to the exact Python minor used by `mgc run`/`mgc dev`, then run `mgc install --core {owner_core}`"
    )]
    IncompatibleRuntimeWheel {
        name: String,
        version: String,
        owner_core: String,
    },
    #[error(
        "Python dependency '{name}'@{version} has no cached wheel for runtime verification: {source}"
    )]
    CachedWheelUnavailable {
        name: String,
        version: String,
        #[source]
        source: io::Error,
    },
    #[error("Python wheel cache entry '{path}' is not a regular file")]
    CachedWheelNotRegularFile { path: String },
    #[error(
        "Python dependency '{name}'@{version} is locked but has no importable MagiCore materialization; compiled wheels/sdists are not yet supported by the native Python runtime"
    )]
    MaterializationMissing { name: String, version: String },
}

impl From<PythonWheelMaterializationError> for MgError {
    fn from(error: PythonWheelMaterializationError) -> Self {
        match error {
            PythonWheelMaterializationError::Shared(error) => error,
            error @ (PythonWheelMaterializationError::MissingIntegrity { .. }
            | PythonWheelMaterializationError::CachedWheelUnavailable { .. }
            | PythonWheelMaterializationError::CachedWheelNotRegularFile { .. }) => {
                Self::Integrity(error.to_string())
            }
            error => Self::Other(error.to_string()),
        }
    }
}
