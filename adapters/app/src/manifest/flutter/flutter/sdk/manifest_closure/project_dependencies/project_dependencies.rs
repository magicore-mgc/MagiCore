//! Read project package names and their runtime/dev classification.
//! Đọc tên package và phân loại runtime/dev của dự án.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use std::collections::BTreeMap;
use std::path::Path;

/// Return the runtime/dev state for project dependencies.
/// Trả trạng thái runtime/dev của dependency dự án.
pub(in crate::manifest::flutter::flutter::sdk::manifest_closure) fn project_dependency_groups(
    project_root: &Path,
) -> FlutterResult<BTreeMap<String, bool>> {
    let project_manifest = mgc_adapter_base::project_file::read_regular_text(
        &project_root.join("pubspec.yaml"),
        "pubspec.yaml",
    )?;
    let project_yaml: serde_yaml::Value =
        serde_yaml::from_str(&project_manifest).map_err(|source| FlutterError::YamlParse {
            context: "parse pubspec.yaml".to_string(),
            source,
        })?;
    let mut project_dependencies = BTreeMap::new();
    for (section, dev) in [("dependencies", false), ("dev_dependencies", true)] {
        if let Some(dependencies) = project_yaml
            .get(section)
            .and_then(serde_yaml::Value::as_mapping)
        {
            for name in dependencies.keys().filter_map(serde_yaml::Value::as_str) {
                project_dependencies
                    .entry(name.to_string())
                    .and_modify(|existing_dev| *existing_dev &= dev)
                    .or_insert(dev);
            }
        }
    }
    Ok(project_dependencies)
}
