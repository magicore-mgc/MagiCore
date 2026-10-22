//! Builds and atomically writes Dart package configuration files.
//! Dựng và ghi atomic các file cấu hình package Dart.

use crate::install::atomic_write_project_file;
use mgc_types::ResolvedGraph;
use std::path::Path;

use super::super::package_graph::build_flutter_package_graph;
use super::super::project_uri::flutter_project_root_uri;
use super::super::types::{FlutterPackageConfig, FlutterPackageConfigEntry};
use super::error::FlutterPackageConfigError;
use super::{registry_entries, sdk_entries};

pub(super) fn write_flutter_package_config_with_sdk_root_inner(
    graph: &ResolvedGraph,
    project_root: &Path,
    pub_cache: &Path,
    sdk_root: Option<&Path>,
) -> Result<(), FlutterPackageConfigError> {
    let config_dir = project_root.join(".dart_tool");
    match std::fs::symlink_metadata(&config_dir) {
        Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => {
            return Err(FlutterPackageConfigError::UnsafeDartTool);
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(FlutterPackageConfigError::InspectDartTool(error));
        }
    }
    let sdk_packages = match sdk_root {
        Some(root) => crate::manifest::flutter::flutter_sdk_package_names(project_root, root)?,
        None => Vec::new(),
    };
    let package_graph = build_flutter_package_graph(graph, project_root, sdk_root, &sdk_packages)?;
    let root_name = package_graph
        .roots
        .first()
        .cloned()
        .ok_or(FlutterPackageConfigError::MissingGraphRoot)?;
    let mut entries = Vec::with_capacity(graph.packages.len() + sdk_packages.len() + 1);
    let mut seen = std::collections::BTreeSet::new();
    if !seen.insert(root_name.clone()) {
        return Err(FlutterPackageConfigError::DuplicatePackage(root_name));
    }
    // Match the directory identity Flutter exposes, including Windows 8.3 expansion above.
    // Khớp danh tính thư mục Flutter thấy, gồm bước bung tên 8.3 ở Windows bên trên.
    let current_dir = if project_root.is_absolute() {
        None
    } else {
        Some(std::env::current_dir().map_err(FlutterPackageConfigError::ReadCurrentDirectory)?)
    };
    let root_uri = flutter_project_root_uri(project_root, current_dir.as_deref())?;
    entries.push(FlutterPackageConfigEntry {
        name: root_name,
        root_uri,
        package_uri: "lib/",
        language_version: None,
    });
    entries.extend(registry_entries::build_registry_entries(
        graph, pub_cache, &mut seen,
    )?);
    entries.extend(sdk_entries::build_sdk_entries(
        &sdk_packages,
        sdk_root,
        &mut seen,
    )?);
    entries.sort_by(|left, right| left.name.cmp(&right.name));
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir).map_err(FlutterPackageConfigError::CreateDartTool)?;
    }
    let config = FlutterPackageConfig {
        config_version: 2,
        generator: "MagiCore",
        generator_version: env!("CARGO_PKG_VERSION"),
        packages: entries,
    };
    let bytes =
        serde_json::to_vec_pretty(&config).map_err(FlutterPackageConfigError::SerializeConfig)?;
    atomic_write_project_file(
        &config_dir.join("package_config.json"),
        &bytes,
        "Flutter package config",
    )?;
    let graph_bytes = serde_json::to_vec_pretty(&package_graph)
        .map_err(FlutterPackageConfigError::SerializeGraph)?;
    Ok(atomic_write_project_file(
        &config_dir.join("package_graph.json"),
        &graph_bytes,
        "Flutter package graph",
    )?)
}
