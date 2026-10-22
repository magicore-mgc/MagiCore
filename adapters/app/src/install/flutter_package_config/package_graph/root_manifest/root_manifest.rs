//! Read the root pubspec identity and dependency declarations.
//! Đọc identity và khai báo dependency trong pubspec gốc.

use super::super::FlutterPackageGraphError;
use super::flutter_pubspec_dependency_names;
use std::path::Path;

pub(in crate::install::flutter_package_config::package_graph) struct RootManifest {
    pub(in crate::install::flutter_package_config::package_graph) name: String,
    pub(in crate::install::flutter_package_config::package_graph) version: String,
    pub(in crate::install::flutter_package_config::package_graph) dependencies: Vec<String>,
    pub(in crate::install::flutter_package_config::package_graph) dev_dependencies: Vec<String>,
}

pub(in crate::install::flutter_package_config::package_graph) fn read_root_manifest(
    project_root: &Path,
) -> Result<RootManifest, FlutterPackageGraphError> {
    let pubspec_path = project_root.join("pubspec.yaml");
    let pubspec_text = std::fs::read_to_string(&pubspec_path)
        .map_err(FlutterPackageGraphError::ReadRootPubspec)?;
    let pubspec: serde_yaml::Value =
        serde_yaml::from_str(&pubspec_text).map_err(FlutterPackageGraphError::ParseRootPubspec)?;
    let name = pubspec
        .get("name")
        .and_then(serde_yaml::Value::as_str)
        .filter(|name| !name.is_empty())
        .ok_or(FlutterPackageGraphError::InvalidRootName)?
        .to_string();
    let version = pubspec
        .get("version")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or("0.0.0")
        .to_string();
    let dependencies = flutter_pubspec_dependency_names(&pubspec, "dependencies")?;
    let dev_dependencies = flutter_pubspec_dependency_names(&pubspec, "dev_dependencies")?;
    Ok(RootManifest {
        name,
        version,
        dependencies,
        dev_dependencies,
    })
}
