//! Typed local errors for Lib dependency resolution.
//! Lỗi có kiểu riêng cho resolver dependency của Lib.

use mgc_types::MgError;

/// Resolver-local failures which must preserve the adapter's fail-closed behavior.
/// Lỗi riêng resolver cần giữ nguyên hành vi fail-closed của adapter.
#[derive(Debug, thiserror::Error)]
pub(super) enum DependencyResolverError {
    /// The pending lock is poisoned and cannot be trusted for replacement.
    /// Pending lock bị poison nên không thể tin cậy để thay thế.
    #[error("lib pending lock poisoned — refusing rather than operating on uncertain state")]
    PendingLockPoisoned,
}

impl From<DependencyResolverError> for MgError {
    fn from(error: DependencyResolverError) -> Self {
        Self::Other(error.to_string())
    }
}
