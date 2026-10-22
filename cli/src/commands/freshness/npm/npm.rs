//! Owns npm name/spec projection, supported semver filtering and Web dev root checks. — Sở hữu projection tên/spec npm, lọc semver hỗ trợ và check root cho Web dev.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

use super::super::lock::{check_declared_roots, locked_root_versions};

/// Web dev freshness includes both direct-root identity and supported npm
/// semver constraints. Non-semver references retain root-presence validation
/// because their source identity is not represented as a version range.
/// (Freshness web dev so cả root trực tiếp lẫn range semver npm đã hỗ trợ.
/// Reference không phải semver chỉ kiểm tra root vì lock chưa biểu diễn source.)
pub fn check_dev_npm_freshness_against_lock(
    project_root: &Path,
    declared: &BTreeMap<String, Vec<String>>,
) -> Result<()> {
    let owner = crate::commands::core::shared::lock_owner_core(project_root, "web")?;
    let locked = locked_root_versions(project_root, &owner, mgc_lockfile::EcosystemTag::Web)?;
    let declared_names = declared.keys().cloned().collect::<Vec<_>>();
    let locked_names = locked.keys().cloned().collect::<Vec<_>>();
    check_declared_roots(&declared_names, &locked_names)?;

    for (name, raw_ranges) in declared {
        let parsed_ranges = raw_ranges
            .iter()
            .filter_map(|raw_range| {
                let range = raw_range.strip_prefix("workspace:").unwrap_or(raw_range);
                npm_range_is_semver(range).then_some((raw_range, range))
            })
            .map(|(raw_range, range)| {
                mgc_types::VersionRange::parse(range)
                    .map(|parsed| (raw_range, parsed))
                    .map_err(|error| {
                        crate::error::freshness_manifest_invalid(
                            "package.json",
                            &format!(
                                "dependency '{name}' has invalid version range '{raw_range}': {error}"
                            ),
                        )
                    })
            })
            .collect::<Result<Vec<_>>>()?;
        if parsed_ranges.is_empty() {
            continue;
        }
        let versions = locked.get(name).map(Vec::as_slice).unwrap_or_default();
        // Duplicate declarations across package sections constrain one
        // resolved direct root; one locked version must satisfy every
        // semver range, rather than letting separate versions satisfy each.
        // (Khai báo trùng tên ở nhiều section cùng ràng buộc một root; một
        // version trong lock phải thỏa mọi range semver.)
        let satisfies = versions.iter().any(|locked_version| {
            mgc_types::Version::parse(locked_version).is_ok_and(|version| {
                parsed_ranges
                    .iter()
                    .all(|(_, range)| range.matches(&version))
            })
        });
        if !satisfies {
            let locked_versions = if versions.is_empty() {
                "none".to_string()
            } else {
                versions.join(", ")
            };
            let declared_ranges = raw_ranges.join(" and ");
            return Err(crate::error::stale_lock_needs_install(&format!(
                "'{name}' declares '{declared_ranges}', but locked version(s) are {locked_versions}"
            )));
        }
    }
    Ok(())
}
fn npm_range_is_semver(range: &str) -> bool {
    let range = range.trim();
    if range.is_empty() || range == "*" || range.eq_ignore_ascii_case("x") {
        return true;
    }
    if range
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        return true;
    }
    if let Some(rest) = range.strip_prefix('v').or_else(|| range.strip_prefix('V'))
        && rest
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
    {
        return true;
    }
    matches!(
        range.chars().next(),
        Some('^' | '~' | '>' | '<' | '=' | '|')
    )
}
/// Declared npm dependencies from `package.json` (all four sections).
/// Dependency npm khai báo từ `package.json` (cả bốn mục).
pub fn declared_npm_deps(manifest: &serde_json::Value) -> Vec<String> {
    let mut declared = Vec::new();
    if let Some(obj) = manifest.as_object() {
        // Spec rev4 §8: the compared set is dependency metadata.
        // (Spec rev4 §8: tập so sánh là metadata dependency.)
        for section in [
            "dependencies",
            "devDependencies",
            "peerDependencies",
            "optionalDependencies",
        ] {
            if let Some(deps) = obj.get(section).and_then(|v| v.as_object()) {
                declared.extend(deps.keys().cloned());
            }
        }
    }
    declared.sort();
    declared.dedup();
    declared
}
/// Declared npm dependency version specs from all four package sections.
/// Version spec npm khai báo từ cả bốn mục dependency.
pub fn declared_npm_dep_specs(
    manifest: &serde_json::Value,
) -> Result<BTreeMap<String, Vec<String>>> {
    let Some(object) = manifest.as_object() else {
        return Err(crate::error::freshness_manifest_invalid(
            "package.json",
            "must be a JSON object",
        ));
    };
    let mut declared = BTreeMap::new();
    for section in [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ] {
        let Some(value) = object.get(section) else {
            continue;
        };
        let Some(dependencies) = value.as_object() else {
            return Err(crate::error::freshness_manifest_invalid(
                "package.json",
                &format!("'{section}' must be an object"),
            ));
        };
        for (name, value) in dependencies {
            let Some(range) = value.as_str() else {
                return Err(crate::error::freshness_manifest_invalid(
                    "package.json",
                    &format!("dependency '{name}' must have a string version spec"),
                ));
            };
            declared
                .entry(name.clone())
                .or_insert_with(Vec::new)
                .push(range.to_string());
        }
    }
    Ok(declared)
}
