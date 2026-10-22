use super::super::json_file::{read_json_object, write_new_private_file};
use super::super::managed_roots::tool_project_name;
use super::super::native_install::install_with_project_credentials;
use super::super::record_validation::normalize_tool_target;
use crate::commands::package_runtime::{
    registry_project_config, requested_package_bins, select_bin_name,
};
use crate::commands::tool::types::TOOL_SCHEMA;
use crate::commands::tool::types::ToolRecord;
use anyhow::{Result, bail};
use std::path::Path;

pub(in crate::commands::tool) fn install_into_project(
    project_root: &Path,
    data_root: &Path,
    package: &str,
    selector: &str,
    selected_bin: Option<&str>,
    allow_scripts: bool,
) -> Result<ToolRecord> {
    let cwd = std::env::current_dir()?;
    let caller_root =
        mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or_else(|| cwd.clone());
    let mut config = registry_project_config(&caller_root)?;
    config.name = tool_project_name(package);
    config.version = "1.0.0".to_string();
    for registry in &mut config.registries {
        registry.token = None;
        registry.username = None;
        registry.password = None;
    }
    config.save(project_root)?;

    let package_json = serde_json::json!({
        "name": config.name,
        "version": "1.0.0",
        "dependencies": { package: selector }
    });
    write_new_private_file(
        &project_root.join("package.json"),
        serde_json::to_vec_pretty(&package_json)?.as_slice(),
    )?;
    if allow_scripts {
        mgc_ui::warning(
            "tool lifecycle scripts are enabled and run with the current user's permissions",
        );
    }
    install_with_project_credentials(project_root, &caller_root, allow_scripts)?;

    let canonical_root = project_root.canonicalize()?;
    if !canonical_root.starts_with(data_root.join("tools").canonicalize()?) {
        bail!("managed tool project escaped the configured data directory");
    }
    let package_root = project_root
        .join("node_modules")
        .join(package)
        .canonicalize()?;
    if !package_root.starts_with(&canonical_root) {
        bail!("installed tool package escaped its managed project");
    }
    let entries = requested_package_bins(&package_root, package)?;
    let binary = select_bin_name(&entries, package, selected_bin)?;
    let target_relative = entries
        .iter()
        .find(|(name, _)| name == &binary)
        .map(|(_, target)| target.clone())
        .ok_or_else(crate::commands::package_runtime::registry_package_undeclared_bin)?;
    let target = normalize_tool_target(&target_relative)?;
    let canonical_target = package_root.join(&target).canonicalize()?;
    if !canonical_target.starts_with(&package_root) || !canonical_target.is_file() {
        bail!("tool executable target is outside its installed package");
    }
    let installed_manifest = read_json_object(
        &package_root.join("package.json"),
        "installed package manifest",
    )?;
    let installed_version = installed_manifest
        .get("version")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("installed tool package has no version"))?
        .to_string();
    Ok(ToolRecord {
        schema: TOOL_SCHEMA,
        source: "npm".to_string(),
        package: package.to_string(),
        selector: selector.to_string(),
        binary,
        target,
        installed_version,
        project_name: tool_project_name(package),
    })
}
