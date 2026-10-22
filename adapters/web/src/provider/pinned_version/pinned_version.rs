//! `pinned_version.rs` — pinned version.
//! Policy cho version được pin.

use super::super::{npm_timestamp::parse_npm_time, release_age::AgePolicy};
use crate::native;
use mgc_types::PackageName;

/// Verify ONE explicitly selected version (dist-tag pin, add-latest,
/// update target) against the age policy. Unlike list filtering, there
/// is no fallback candidate — a rejected pin is a hard error telling
/// the user to pin an older version explicitly.
/// (Kiểm tra một version đã chọn theo policy tuổi.)
pub fn check_pinned_version(
    package: &PackageName,
    meta: &native::npm_registry::PackageMetadata,
    version: &str,
    policy: Option<AgePolicy>,
) -> Result<(), String> {
    let Some(policy) = policy.filter(|p| p.cutoff_hours > 0) else {
        return Ok(());
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    match meta.time.get(version).and_then(|text| parse_npm_time(text)) {
        Some(ts) if now.saturating_sub(ts) >= policy.cutoff_hours.saturating_mul(3600) => Ok(()),
        Some(_) => Err(format!(
            concat!(
                "minimum-release-age: {}@{} is younger than {}h — pin an older version explicitly ",
                "or relax [security] min_release_age"
            ),
            package.as_str(),
            version,
            policy.cutoff_hours
        )),
        None if policy.allow_missing_time => Ok(()),
        None => Err(format!(
            concat!(
                "minimum-release-age: {}@{} has no registry timestamp — set ",
                "[security] allow_missing_time for private registries"
            ),
            package.as_str(),
            version
        )),
    }
}
