//! Validate the Cargo dependency-table subset supported by the projection.
//! Kiểm tra tập dependency table Cargo được projection hỗ trợ.

use super::super::error::CargoManifestError;
use mgc_lockfile::Package;
use std::collections::BTreeMap;

/// Reject unsupported Cargo options and dependencies missing from `mgc.lock`.
/// Chặn option Cargo chưa hỗ trợ và dependency không có trong `mgc.lock`.
pub(in crate::install::cargo_compat::manifest) fn validate_dependency_specs(
    manifest: &toml::Value,
    by_name: &BTreeMap<&str, Vec<&Package>>,
) -> Result<(), CargoManifestError> {
    if let Some(dependencies) = manifest.get("dependencies") {
        let dependencies = dependencies
            .as_table()
            .ok_or(CargoManifestError::DependenciesTableRequired)?;
        for (name, spec) in dependencies {
            match spec {
                toml::Value::String(_) => {}
                toml::Value::Table(spec) => {
                    if ["path", "git", "workspace", "package", "registry"]
                        .iter()
                        .any(|key| spec.contains_key(*key))
                    {
                        return Err(CargoManifestError::UnsupportedDependencyOrigin {
                            name: name.clone(),
                        });
                    }
                    if spec.get("optional").and_then(toml::Value::as_bool) == Some(true) {
                        return Err(CargoManifestError::OptionalDependencyUnsupported {
                            name: name.clone(),
                        });
                    }
                    if spec.get("version").and_then(toml::Value::as_str).is_none() {
                        return Err(CargoManifestError::DependencyVersionRequired {
                            name: name.clone(),
                        });
                    }
                    if let Some(features) = spec.get("features")
                        && features.as_array().is_none_or(|features| {
                            features
                                .iter()
                                .any(|feature| feature.as_str().is_none_or(str::is_empty))
                        })
                    {
                        return Err(CargoManifestError::DependencyFeaturesInvalid {
                            name: name.clone(),
                        });
                    }
                    if let Some(default_features) = spec.get("default-features")
                        && default_features.as_bool().is_none()
                    {
                        return Err(CargoManifestError::DependencyDefaultFeaturesInvalid {
                            name: name.clone(),
                        });
                    }
                    if spec.keys().any(|key| {
                        !matches!(
                            key.as_str(),
                            "version" | "features" | "default-features" | "optional"
                        )
                    }) {
                        return Err(CargoManifestError::UnknownDependencyOption {
                            name: name.clone(),
                        });
                    }
                }
                _ => {
                    return Err(CargoManifestError::DependencySpecInvalid { name: name.clone() });
                }
            }
            if !by_name.contains_key(name.as_str()) {
                return Err(CargoManifestError::DependencyMissingFromLock { name: name.clone() });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/dependency_specs.rs"]
mod tests;
