use super::super::atomic_file::atomic_replace_file;
use super::super::json_file::read_json_object;
use super::super::managed_roots::{managed_roots, shim_path};
use super::super::native_install::{install_with_project_credentials, require_web_lane};
use super::super::record_lookup::find_record_by_binary;
use super::super::record_validation::normalize_tool_target;
use super::super::record_write::write_record;
use super::super::shim::ensure_shim;
use crate::commands::package_runtime::{parse_request, requested_package_bins, valid_bin_name};
use anyhow::{Context, Result, bail};

pub(in crate::commands::tool) fn update(
    binary: &str,
    selector_override: Option<&str>,
    allow_scripts: bool,
) -> Result<()> {
    require_web_lane()?;
    if !valid_bin_name(binary) {
        return Err(crate::commands::package_runtime::registry_package_invalid_bin());
    }
    let (_data_root, tools_root, bin_root) = managed_roots(false)?;
    let (project_root, mut record) = find_record_by_binary(&tools_root, binary)?
        .ok_or_else(|| anyhow::anyhow!("managed tool '{}' is not installed", binary))?;
    let selector = selector_override.unwrap_or("latest");
    let request = format!("{}@{}", record.package, selector);
    let (parsed_package, parsed_selector) = parse_request(&request)?;
    if parsed_package != record.package {
        bail!("tool update package identity changed unexpectedly");
    }
    let old_record = record.clone();
    record.selector = parsed_selector.to_string();
    let package_json_path = project_root.join("package.json");
    let original_package_json = mgc_config::project::read_regular_project_text(
        &package_json_path,
        "managed tool package manifest",
    )?
    .ok_or_else(|| anyhow::anyhow!("managed tool package manifest is missing"))?;
    let mut package_json: serde_json::Value = serde_json::from_str(&original_package_json)?;
    let dependencies = package_json
        .get_mut("dependencies")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| anyhow::anyhow!("managed tool dependency manifest is malformed"))?;
    if !dependencies.contains_key(&record.package) {
        bail!("managed tool package manifest no longer owns the recorded package");
    }
    dependencies.insert(
        record.package.clone(),
        serde_json::Value::String(record.selector.clone()),
    );
    atomic_replace_file(
        &package_json_path,
        &serde_json::to_vec_pretty(&package_json)?,
    )?;

    let shim = shim_path(&bin_root, binary);
    let update_result = (|| -> Result<()> {
        let cwd = std::env::current_dir()?;
        let caller_root =
            mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or(cwd);
        install_with_project_credentials(&project_root, &caller_root, allow_scripts)?;
        let package_root = project_root
            .join("node_modules")
            .join(&record.package)
            .canonicalize()?;
        let project_root_canonical = project_root.canonicalize()?;
        if !package_root.starts_with(&project_root_canonical) {
            bail!("updated tool package escaped its managed project");
        }
        let entries = requested_package_bins(&package_root, &record.package)?;
        let target = entries
            .iter()
            .find(|(name, _)| name == binary)
            .map(|(_, target)| target.clone())
            .ok_or_else(crate::commands::package_runtime::registry_package_undeclared_bin)?;
        let target = normalize_tool_target(&target)?;
        let canonical_target = package_root.join(&target).canonicalize()?;
        if !canonical_target.starts_with(&package_root) || !canonical_target.is_file() {
            bail!("updated tool executable target is outside its installed package");
        }
        let installed_manifest = read_json_object(
            &package_root.join("package.json"),
            "installed package manifest",
        )?;
        record.target = target;
        record.installed_version = installed_manifest
            .get("version")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("updated tool package has no version"))?
            .to_string();
        ensure_shim(&project_root, &record, &shim, true)?;
        write_record(&project_root, &record)?;
        Ok(())
    })();
    if let Err(error) = update_result {
        let restore = atomic_replace_file(&package_json_path, original_package_json.as_bytes());
        let rollback = if restore.is_ok() {
            let cwd = std::env::current_dir()?;
            let caller_root =
                mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or(cwd);
            install_with_project_credentials(&project_root, &caller_root, allow_scripts)
        } else {
            Ok(())
        };
        let restore_shim = ensure_shim(&project_root, &old_record, &shim, true);
        if let Err(restore_error) = restore {
            return Err(error).context(format!(
                "failed to restore tool manifest: {restore_error:#}"
            ));
        }
        if let Err(rollback_error) = rollback {
            return Err(error).context(format!(
                "tool update rollback also failed: {rollback_error:#}"
            ));
        }
        if let Err(shim_error) = restore_shim {
            return Err(error).context(format!("tool shim rollback also failed: {shim_error:#}"));
        }
        return Err(error).context("tool update failed; the previous version was restored");
    }
    mgc_ui::success(&format!(
        "Updated {} to {} ({})",
        record.binary, record.installed_version, record.package
    ));
    Ok(())
}
