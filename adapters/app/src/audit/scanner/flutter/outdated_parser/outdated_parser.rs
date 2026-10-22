//! Parse pub outdated JSON for dependency-health regression fixtures.
//! Parse JSON pub outdated phục vụ fixture hồi quy dependency-health.

use mgc_types::adapter::{DependencyHealthReport, OutdatedDependency};
use mgc_types::{MgError, MgResult, PackageId, PackageName, Version};
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Legacy parser fixtures retained for unit regression tests only.
// Parser cũ chỉ giữ làm fixture kiểm thử hồi quy.
// ---------------------------------------------------------------------------

/// `flutter pub outdated --json` — top-level object with package groups
/// (camelCase keys per the real Dart pub output).
#[cfg(test)]
#[derive(Debug, Deserialize)]
struct PubOutdatedReport {
    /// At least one recognized group must exist, else the payload is not a
    /// pub outdated report — fail closed instead of reporting zero drift.
    /// Phải có ít nhất 1 group được nhận diện, nếu không payload không phải
    /// report pub outdated — fail-closed thay vì báo 0 drift.
    #[serde(default)]
    packages: Vec<PubOutdatedEntry>,
    #[serde(default, rename = "devPackages")]
    dev_packages: Vec<PubOutdatedEntry>,
    #[serde(default, rename = "dependencyOverrides")]
    dependency_overrides: Vec<PubOutdatedEntry>,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
struct PubOutdatedEntry {
    package: String,
    #[serde(default)]
    current: Option<String>,
    #[serde(default)]
    latest: Option<String>,
}

/// Parse `flutter pub outdated --json` output into a DEPENDENCY HEALTH
/// report — version drift, NOT vulnerabilities.
/// Parse output `flutter pub outdated --json` thành báo cáo ĐỘ TƯƠI
/// dependency — lệch version, KHÔNG phải lỗ hổng.
#[cfg(test)]
pub(crate) fn parse_flutter_outdated_json(raw: &str) -> MgResult<DependencyHealthReport> {
    let json_start = raw.find('{').unwrap_or(0);
    let report: PubOutdatedReport = serde_json::from_str(raw[json_start..].trim())
        .map_err(|e| MgError::Other(format!("invalid flutter pub outdated JSON: {e}")))?;

    let mut checked = 0usize;
    let mut outdated = Vec::new();
    let mut rejected = Vec::new();

    for entry in report
        .packages
        .iter()
        .chain(&report.dev_packages)
        .chain(&report.dependency_overrides)
    {
        checked += 1;
        let Ok(pkg_name) = PackageName::new(&entry.package) else {
            rejected.push(format!("package '{}'", entry.package));
            continue;
        };
        let latest = entry.latest.as_deref().unwrap_or("");
        let current = entry.current.as_deref().unwrap_or("");
        // Entries without a resolvable latest (discontinued/unpublished) are
        // counted as checked but carry no actionable target.
        // Entry không có latest giải được (ngừng publish) tính là đã kiểm
        // tra nhưng không có mục tiêu xử lý.
        if latest.is_empty() {
            continue;
        }
        let (Ok(latest_version), Ok(current_version)) =
            (Version::parse(latest), Version::parse(current))
        else {
            rejected.push(format!(
                "package '{}' versions '{current}'/'{latest}'",
                entry.package
            ));
            continue;
        };
        if current_version == latest_version {
            continue;
        }
        outdated.push(OutdatedDependency {
            package: PackageId::new(pkg_name, current_version),
            latest_version,
        });
    }

    // No recognized group at all → schema error, not "zero drift".
    // Không group nào được nhận diện → lỗi schema, không phải "0 drift".
    if checked == 0 {
        return Err(MgError::Other(
            "flutter pub outdated JSON has no packages/devPackages entries".to_string(),
        ));
    }
    if !rejected.is_empty() {
        return Err(MgError::Other(format!(
            "flutter pub outdated JSON contained {} unparseable entries: {}",
            rejected.len(),
            rejected.join("; ")
        )));
    }

    Ok(DependencyHealthReport {
        packages_checked: checked,
        outdated_count: outdated.len(),
        outdated,
    })
}
