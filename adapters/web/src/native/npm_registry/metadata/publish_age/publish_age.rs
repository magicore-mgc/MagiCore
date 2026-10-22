//! Check publish-age thresholds and report unknown registry timestamps.
//! Kiểm tra ngưỡng tuổi phát hành và báo trạng thái timestamp registry chưa rõ.

use crate::native::npm_registry::PackageMetadata;
use anyhow::Result;

/// Return `Err` for a quarantined version or an unsupported threshold; unknown time is `Ok(false)`.
/// Trả `Err` nếu version còn trong quarantine hoặc ngưỡng không hỗ trợ; timestamp chưa rõ là `Ok(false)`.
pub fn check_publish_age(
    metadata: &PackageMetadata,
    version: &str,
    min_age_secs: i64,
) -> Result<bool, String> {
    if min_age_secs <= 0 {
        return Ok(true);
    }
    let Some(minimum_age) = chrono::TimeDelta::try_seconds(min_age_secs) else {
        return Err("minimum release age exceeds the supported duration range".to_owned());
    };
    let Some(ts_str) = metadata.time.get(version) else {
        // Missing time leaves the policy decision to the caller.
        // Timestamp thiếu để policy ở phía caller quyết định.
        return Ok(false);
    };

    let Ok(published) = chrono::DateTime::parse_from_rfc3339(ts_str) else {
        return Ok(false);
    };
    let now = chrono::Utc::now();
    let age = now.signed_duration_since(published.with_timezone(&chrono::Utc));

    if age < minimum_age {
        let hours = age.num_minutes() as f64 / 60.0;
        let minimum_hours = min_age_secs as f64 / 3600.0;
        Err(format!(
            "🚨 SECURITY: Package '{}@{}' was published only {:.1}h ago (< {:.1}h quarantine).\n   \
             This may be a supply-chain attack. Set MAGICORE_ALLOW_UNTRUSTED=1 to override.",
            metadata.name, version, hours, minimum_hours
        ))
    } else {
        Ok(true)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
