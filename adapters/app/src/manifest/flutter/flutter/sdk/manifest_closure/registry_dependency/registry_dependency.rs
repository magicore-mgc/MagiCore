//! Merge a Flutter SDK registry dependency into the project manifest.
//! Gộp dependency registry từ Flutter SDK vào manifest dự án.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use mgc_types::{DependencySpec, Manifest, PackageName, VersionRange};
use std::collections::BTreeMap;

/// Intersect supported range constraints and preserve the effective dependency group.
/// Gộp constraint được hỗ trợ và giữ đúng nhóm dependency hiệu lực.
pub(in crate::manifest::flutter::flutter::sdk::manifest_closure) fn merge_registry_dependency(
    package_name: &str,
    dependency_name: &str,
    value: &serde_yaml::Value,
    dev: bool,
    project_dependencies: &BTreeMap<String, bool>,
    registry_ranges: &mut BTreeMap<String, (VersionRange, bool)>,
    manifest: &mut Manifest,
) -> FlutterResult<()> {
    let raw_range = match value {
        serde_yaml::Value::String(range) if !range.trim().is_empty() => range.as_str(),
        serde_yaml::Value::Null => "*",
        _ => {
            return Err(FlutterError::Unsupported {
                capability: "Flutter SDK package dependency constraint",
                guidance: format!(
                    "Flutter SDK package '{package_name}' has an unsupported constraint for '{dependency_name}'"
                ),
            });
        }
    };
    let package = PackageName::new(dependency_name).map_err(|error| FlutterError::Unsupported {
        capability: "Flutter SDK dependency name",
        guidance: format!("MagiCore cannot safely resolve '{dependency_name}': {error}"),
    })?;
    let requested_range =
        VersionRange::parse(raw_range).map_err(|error| FlutterError::Unsupported {
            capability: "Flutter SDK dependency range",
            guidance: format!("MagiCore cannot safely resolve '{dependency_name}': {error}"),
        })?;
    let previous = registry_ranges
        .get(dependency_name)
        .map(|(range, _)| range)
        .or_else(|| manifest.find_dep(dependency_name).map(|dep| &dep.range));
    let combined = match previous {
        Some(previous) if previous.is_star() => requested_range.to_string(),
        Some(previous) if requested_range.is_star() => previous.to_string(),
        Some(previous) if previous == &requested_range => previous.to_string(),
        Some(previous)
            if previous.as_str().contains("||") || requested_range.as_str().contains("||") =>
        {
            return Err(FlutterError::Unsupported {
                capability: "Flutter SDK disjunctive dependency constraints",
                guidance: format!(
                    "MagiCore cannot safely intersect disjunctive constraints for '{dependency_name}' from the project and Flutter SDK metadata"
                ),
            });
        }
        Some(previous) => format!("{} {}", previous.as_str(), requested_range.as_str()),
        None => requested_range.to_string(),
    };
    let range = VersionRange::parse(&combined)?;
    let existing_sdk_dev = registry_ranges
        .get(dependency_name)
        .map(|(_, existing_dev)| *existing_dev)
        .unwrap_or(true);
    // A dependency is dev-only only when every incoming edge is dev-only. A runtime edge from the Flutter SDK must promote a
    // project dev dependency into the runtime group.
    // (Chỉ dev khi mọi cạnh đều là dev; SDK dùng runtime phải promote.)
    let project_dev = project_dependencies
        .get(dependency_name)
        .copied()
        .unwrap_or(true);
    let effective_dev = project_dev && existing_sdk_dev && dev;
    registry_ranges.insert(dependency_name.to_string(), (range.clone(), effective_dev));
    manifest.add_dep(
        DependencySpec::new(package, range),
        effective_dev,
        false,
        false,
    );
    Ok(())
}
