//! `eligible_versions.rs` — release-age candidate filter / lọc version theo tuổi release.
//! Lọc candidate version theo age gate.

use super::super::npm_timestamp::parse_npm_time;
use super::super::release_age::AgePolicy;
use crate::native;
use mgc_types::{PackageName, Version};

/// Filter registry candidates for every resolution, prefetch, pin and update path.
/// `None` keeps historical behavior; an active policy fails closed for unstamped versions.
/// (Hàm eligible duy nhất — mọi đường chọn version đi qua đây.)
pub fn eligible_versions(
    package: &PackageName,
    meta: &native::npm_registry::PackageMetadata,
    policy: Option<AgePolicy>,
) -> Result<Vec<Version>, String> {
    let all: Vec<Version> = meta
        .versions
        .keys()
        .filter_map(|v| Version::parse(v).ok())
        .collect();
    let Some(policy) = policy.filter(|p| p.cutoff_hours > 0) else {
        return Ok(all);
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let mut kept = Vec::new();
    let mut young = 0usize;
    let mut unstamped = 0usize;
    for version in all {
        let key = version.to_string();
        match meta.time.get(&key).and_then(|text| parse_npm_time(text)) {
            Some(ts) if now.saturating_sub(ts) >= policy.cutoff_hours.saturating_mul(3600) => {
                kept.push(version);
            }
            Some(_) => {
                young += 1;
            }
            None => {
                if policy.allow_missing_time {
                    kept.push(version);
                } else {
                    unstamped += 1;
                }
            }
        }
    }
    if kept.is_empty() && !meta.versions.is_empty() {
        return Err(format!(
            concat!(
                "minimum-release-age: all {} version(s) of {} excluded ({} too young, {} unstamped) — ",
                "relax [security] min_release_age, set allow_missing_time, or pin an older version explicitly"
            ),
            meta.versions.len(),
            package.as_str(),
            young,
            unstamped
        ));
    }
    if young > 0 || unstamped > 0 {
        eprintln!(
            "[magicore] minimum-release-age: excluded {} young + {} unstamped version(s) of {} (cutoff {}h)",
            young,
            unstamped,
            package.as_str(),
            policy.cutoff_hours
        );
    }
    Ok(kept)
}
