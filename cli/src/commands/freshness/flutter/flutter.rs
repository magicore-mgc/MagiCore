//! Owns pubspec projection and app-feature-aware Flutter SDK-expanded lock checks. — Sở hữu projection pubspec và kiểm tra lock Flutter mở rộng SDK theo feature app.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

use super::super::lock::{check_declared_roots, locked_root_versions};
use super::super::manifest_syntax::validate_manifest_syntax;

/// Declared Flutter dependencies from `pubspec.yaml` top-level mapping.
/// Dependency Flutter khai báo từ mapping top-level `pubspec.yaml`.
// Keep the public projection API for library consumers and non-app builds.
// Giữ API projection cho library; binary app dùng manifest đã mở rộng từ SDK.
#[cfg_attr(feature = "app", allow(dead_code))]
pub fn declared_flutter_deps(pubspec_yaml: &str) -> Vec<String> {
    let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(pubspec_yaml) else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for section in ["dependencies", "dev_dependencies"] {
        if let Some(map) = value.get(section).and_then(|v| v.as_mapping()) {
            names.extend(map.keys().filter_map(|k| k.as_str()).map(|s| s.to_string()));
        }
    }
    names.sort();
    names.dedup();
    names
}
/// Flutter freshness for app/dev flows (`pubspec.yaml`).
/// Freshness Flutter cho luồng app/dev (`pubspec.yaml`).
pub fn check_flutter_freshness(project_root: &Path) -> Result<()> {
    check_flutter_manifest_freshness(project_root, true)?;
    Ok(())
}
/// Compare the same SDK-expanded registry roots used by native install.
/// So cùng root registry đã mở rộng từ SDK, không coi package SDK là pin Pub.
pub(in crate::commands::freshness) fn check_flutter_manifest_freshness(
    project_root: &Path,
    require_lock: bool,
) -> Result<bool> {
    #[cfg(not(feature = "app"))]
    {
        check_manifest_file(
            project_root,
            "pubspec.yaml",
            require_lock,
            "app",
            mgc_lockfile::EcosystemTag::Dart,
            |content| Ok(declared_flutter_deps(content)),
        )
    }
    #[cfg(feature = "app")]
    {
        use mgc_types::adapter::PackageAdapter;

        let Some(content) = mgc_config::project::read_regular_project_text(
            &project_root.join("pubspec.yaml"),
            "pubspec.yaml",
        )?
        else {
            return Ok(false);
        };
        let has_lock = project_root.join("mgc.lock").exists();
        if require_lock && !has_lock {
            return Err(crate::error::stale_lock_needs_install(
                "mgc.lock is missing",
            ));
        }
        validate_manifest_syntax("pubspec.yaml", &content)?;
        if !has_lock {
            return Ok(true);
        }
        let manifest = mgc_app_adapter::manifest::flutter::parse_pubspec(project_root)?;
        let adapter = mgc_app_adapter::AppAdapter::new(mgc_app_adapter::AppLanguage::Flutter);
        let effective = adapter.lock_replay_manifest(project_root, &manifest)?;
        let owner = crate::commands::core::shared::lock_owner_core(project_root, "app")?;
        let locked = locked_root_versions(project_root, &owner, mgc_lockfile::EcosystemTag::Dart)?;
        let mut declared = BTreeMap::<String, Vec<&mgc_types::VersionRange>>::new();
        for dependency in effective.all_dependencies() {
            declared
                .entry(dependency.name.to_string())
                .or_default()
                .push(&dependency.range);
        }
        let names = declared.keys().cloned().collect::<Vec<_>>();
        check_declared_roots(&names, &locked.keys().cloned().collect::<Vec<_>>())?;
        for (name, ranges) in declared {
            let versions = locked.get(&name).map(Vec::as_slice).unwrap_or_default();
            // One direct version must satisfy every project and SDK constraint.
            // Một version trực tiếp phải thỏa mọi constraint của project và SDK.
            let satisfies = versions.iter().any(|version| {
                mgc_types::Version::parse(version)
                    .is_ok_and(|version| ranges.iter().all(|range| range.matches(&version)))
            });
            if !satisfies {
                let requirements = ranges
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" and ");
                return Err(crate::error::stale_lock_needs_install(&format!(
                    "'{name}' declares '{requirements}', but locked version(s) are {}",
                    versions.join(", ")
                )));
            }
        }
        Ok(true)
    }
}
