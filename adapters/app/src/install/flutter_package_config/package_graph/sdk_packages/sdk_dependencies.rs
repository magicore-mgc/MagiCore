//! Project only SDK dependency forms supported by Dart package_graph.
//! Chỉ projection dạng dependency SDK Dart package_graph hỗ trợ.

use mgc_types::MgError;

use super::super::FlutterPackageGraphError;

pub(super) fn sdk_dependency_names(
    package_name: &str,
    manifest: &serde_yaml::Value,
) -> Result<Vec<String>, FlutterPackageGraphError> {
    let mut dependencies = Vec::new();
    if let Some(dependency_map) = manifest.get("dependencies") {
        let mapping = dependency_map.as_mapping().ok_or_else(|| {
            FlutterPackageGraphError::SdkDependenciesNotMapping(package_name.to_string())
        })?;
        for (dependency_name, value) in mapping {
            let dependency_name = dependency_name
                .as_str()
                .ok_or(FlutterPackageGraphError::NonStringSdkDependency)?;
            if let Some(fields) = value.as_mapping() {
                let source = fields
                    .get(serde_yaml::Value::String("sdk".to_string()))
                    .and_then(serde_yaml::Value::as_str)
                    .ok_or_else(|| MgError::Unsupported {
                        core: "app",
                        capability: "Flutter package graph dependency source",
                        guidance: format!(
                            "cannot represent dependency source for '{dependency_name}' in SDK package '{package_name}'"
                        ),
                    })?;
                if fields.len() != 1 || !matches!(source, "flutter" | "dart") {
                    return Err(FlutterPackageGraphError::Shared(MgError::Unsupported {
                        core: "app",
                        capability: "Flutter package graph dependency source",
                        guidance: format!(
                            "cannot represent source '{source}' for '{dependency_name}' in SDK package '{package_name}'"
                        ),
                    }));
                }
                if source == "flutter" {
                    dependencies.push(dependency_name.to_string());
                }
            } else if value.is_string() || value.is_null() {
                dependencies.push(dependency_name.to_string());
            } else {
                return Err(FlutterPackageGraphError::Shared(MgError::Unsupported {
                    core: "app",
                    capability: "Flutter package graph dependency constraint",
                    guidance: format!(
                        "cannot represent dependency '{dependency_name}' in SDK package '{package_name}'"
                    ),
                }));
            }
        }
    }
    dependencies.sort();
    dependencies.dedup();
    Ok(dependencies)
}
