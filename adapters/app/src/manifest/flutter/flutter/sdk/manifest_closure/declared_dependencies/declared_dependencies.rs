//! Parse Flutter SDK packages declared by the project manifest.
//! Đọc package Flutter SDK được khai báo trong manifest dự án.

use crate::manifest::flutter::{FlutterError, FlutterResult};
use std::collections::BTreeMap;
use std::path::Path;

/// Read direct Flutter SDK dependencies and preserve runtime/dev grouping.
/// Đọc dependency SDK trực tiếp và giữ nhóm runtime/dev.
pub(in crate::manifest::flutter::flutter::sdk) fn declared_flutter_sdk_dependencies(
    project_root: &Path,
) -> FlutterResult<Vec<(String, bool)>> {
    let path = project_root.join("pubspec.yaml");
    let content = mgc_adapter_base::project_file::read_regular_text(&path, "pubspec.yaml")?;
    let pubspec: serde_yaml::Value =
        serde_yaml::from_str(&content).map_err(|source| FlutterError::YamlParse {
            context: "parse pubspec.yaml".to_string(),
            source,
        })?;
    let Some(root) = pubspec.as_mapping() else {
        return Err(FlutterError::Integrity(
            "pubspec.yaml must be a mapping".to_string(),
        ));
    };
    let mut packages = BTreeMap::new();
    for (section, dev) in [("dependencies", false), ("dev_dependencies", true)] {
        let Some(dependencies) = root
            .get(serde_yaml::Value::String(section.to_string()))
            .and_then(serde_yaml::Value::as_mapping)
        else {
            continue;
        };
        for (name, value) in dependencies {
            let Some(fields) = value.as_mapping() else {
                continue;
            };
            let Some(sdk) = fields
                .get(serde_yaml::Value::String("sdk".to_string()))
                .and_then(serde_yaml::Value::as_str)
            else {
                continue;
            };
            if sdk != "flutter" || fields.len() != 1 {
                return Err(FlutterError::Unsupported {
                    capability: "non-Flutter SDK dependency",
                    guidance: "MagiCore can currently bind only dependencies supplied by the selected Flutter SDK".to_string(),
                });
            }
            let name = name.as_str().ok_or_else(|| {
                FlutterError::Integrity("Flutter SDK dependency name must be a string".to_string())
            })?;
            super::super::super::package_root::validate_flutter_sdk_package_name(name)?;
            packages
                .entry(name.to_string())
                .and_modify(|existing_dev| *existing_dev &= dev)
                .or_insert(dev);
        }
    }
    Ok(packages.into_iter().collect())
}
