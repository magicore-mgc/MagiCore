//! Typed loader failures mapped to the legacy parse-error contract.
//! Lỗi loader có kiểu, ánh xạ về contract ParseError hiện tại.

/// Fail-closed loader errors specific to the legacy/v3 API boundary.
/// Lỗi fail-closed riêng tại boundary API legacy/v3.
#[derive(Debug, thiserror::Error)]
pub(super) enum LockLoadError {
    #[error(
        "v4 lockfile requires the lossless LockDocument API; legacy Lockfile cannot preserve v4 package identities"
    )]
    LossyLegacyModel,
    #[error(
        "mgc.lock '{path}' is a v4 lockfile but this operation runs under v3 schema: set mgc.toml [lock] schema = \"v4\" or re-install to rewrite it"
    )]
    V4UnderV3 { path: String },
}

impl From<LockLoadError> for crate::LockfileError {
    fn from(error: LockLoadError) -> Self {
        Self::ParseError(error.to_string())
    }
}
