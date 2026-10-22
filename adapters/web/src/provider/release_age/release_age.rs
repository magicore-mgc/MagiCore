//! `release_age.rs` — release-age policy type / kiểu policy tuổi release.
//! Policy tuổi release.

use super::super::{eligible_versions::eligible_versions, provider::NpmDependencyProvider};
use crate::native;
use mgc_resolver::DependencyError;
use mgc_types::{PackageName, Version};

/// Age-gate policy and missing-timestamp behavior.
/// Policy tuổi và cách xử lý timestamp bị thiếu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgePolicy {
    /// Minimum age in hours.
    pub cutoff_hours: u64,
    /// Keep versions with missing/unparsable timestamps (private
    /// registries). Default false.
    pub allow_missing_time: bool,
}

impl NpmDependencyProvider {
    /// Version list with the minimum-age gate applied — the ONLY place
    /// version lists are built (get_versions + all prefetch paths funnel
    /// here, so the gate cannot be bypassed by a second listing site).
    /// Gate rejections propagate as precise errors (never a confusing
    /// downstream "no version matches").
    /// (Dựng danh sách version kèm cổng tuổi — điểm duy nhất.)
    pub(crate) fn versions_with_age_gate(
        &self,
        package: &PackageName,
        meta: &native::npm_registry::PackageMetadata,
    ) -> Result<Vec<Version>, DependencyError> {
        eligible_versions(package, meta, self.age_policy()).map_err(DependencyError)
    }
}
