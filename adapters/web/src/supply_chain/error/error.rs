//! Typed refusals produced by Web supply-chain guard owners.
//! Các refusal có kiểu do owner guard supply-chain Web tạo ra.

use std::path::PathBuf;

use mgc_types::{MgError, Version};

/// Guard errors remain local; the coordinator maps their Display text to legacy MgError.
/// Lỗi guard giữ cục bộ; coordinator ánh xạ Display về MgError tương thích.
#[derive(Debug, thiserror::Error)]
pub(in crate::supply_chain) enum SupplyChainError {
    #[error("{message}")]
    PublishAge { message: String },
    #[error("cannot determine project directory for supply-chain policy: {source}")]
    ProjectRoot {
        #[source]
        source: std::io::Error,
    },
    #[error("cannot read project security policy at {path}: {source}")]
    ProjectSecurityConfig {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error("{unit} release-age value in {path} is outside the supported range: {value}")]
    MinimumAgeOutOfRange {
        path: PathBuf,
        value: u64,
        unit: &'static str,
    },
    #[error("release-age threshold is outside the supported duration range: {seconds} seconds")]
    MinimumAgeSecondsOutOfRange { seconds: i64 },
    #[error("cannot inspect release-policy database at {path}: {source}")]
    StorePolicyInspect {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot read release-policy database at {path}: {source}")]
    StorePolicyDatabase {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error(
        "resolved package '{name}@{version}' has no registry metadata for the active age guard"
    )]
    MissingPackageMetadata { name: String, version: String },
    #[error(
        "publish timestamp is missing for '{name}@{version}'; \
         set [security] allow_missing_time = true only for registries that omit timestamps"
    )]
    MissingPublishTimestamp { name: String, version: String },
    #[error(
        "publish timestamp is invalid for '{name}@{version}'; \
         set [security] allow_missing_time = true only for registries that omit valid timestamps"
    )]
    InvalidPublishTimestamp { name: String, version: String },
    #[error("cannot read project lockfile for downgrade guard: {source}")]
    LockfileRead {
        #[source]
        source: MgError,
    },
    #[error(
        "lockfile has an invalid version for direct dependency '{name}': '{version}': {source}"
    )]
    InvalidLockfileVersion {
        name: String,
        version: String,
        #[source]
        source: Box<MgError>,
    },
    #[error("lockfile has an invalid direct root pin '{pin}': {source}")]
    InvalidRootPin {
        pin: String,
        #[source]
        source: MgError,
    },
    #[error("lockfile direct root pin '{package_id}' has no matching owner-scoped Web package row")]
    MissingLockedRootPackage { package_id: String },
    #[error("lockfile has multiple direct root pins for dependency '{name}'")]
    AmbiguousDirectRootPins { name: String },
    #[error("resolver returned no direct package for root request {index} ('{name}')")]
    MissingRootResolution { index: usize, name: String },
    #[error(
        "shared resolution graph has multiple direct packages for root request {index} ('{name}')"
    )]
    AmbiguousCachedRootResolution { index: usize, name: String },
    #[error("resolver returned duplicate direct package mappings for root request {index}")]
    DuplicateRootResolution { index: usize },
    #[error("resolver returned {actual} direct package mappings; expected {expected}")]
    RootResolutionCount { expected: usize, actual: usize },
    #[error(
        "resolver direct mapping for request {index} does not match manifest \
         ('{expected_name}', request '{actual_request_name}', package '{actual_package_name}')"
    )]
    InvalidRootResolution {
        index: usize,
        expected_name: String,
        actual_request_name: String,
        actual_package_name: String,
    },
    #[error(
        "🚨 SECURITY: Downgrade blocked for '{name}' — previous top-level {previous}, \
         new top-level selection {selected}.\n   Use MAGICORE_ALLOW_UNTRUSTED=1 to override."
    )]
    Downgrade {
        name: String,
        previous: Box<Version>,
        selected: Box<Version>,
    },
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
