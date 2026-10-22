//! Shared Cargo dependency-manifest API.
//! API dùng chung để quản lý dependency trong Cargo manifest.

mod cargo_manifest;
mod dependency_table;
mod error;
mod parse_manifest;
mod write_manifest;

pub use self::cargo_manifest::{parse_manifest, write_manifest};
pub use self::error::{CargoManifestError, CargoManifestErrorKind, CargoManifestRecovery};
