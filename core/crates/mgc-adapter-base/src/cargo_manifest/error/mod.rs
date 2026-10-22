//! Cargo manifest error types and conversion helpers.
//! Type lỗi Cargo manifest và helper chuyển lỗi.

mod error;

pub use self::error::{CargoManifestError, CargoManifestErrorKind, CargoManifestRecovery};
pub(super) use self::error::{
    invalid_manifest, parse_error, serialization_error, unsupported_manifest, unsupported_mutation,
};
