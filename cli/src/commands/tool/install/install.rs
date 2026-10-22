use super::super::managed_roots::{
    managed_roots, set_private_directory, shim_path, tool_project_name,
};
use super::super::native_install::require_web_lane;
use super::super::project_install::install_into_project;
use super::super::record_validation::{read_pending_if_present, read_record_if_present};
use super::super::record_write::{write_pending, write_record};
use super::super::shim::{ensure_shim, remove_owned_shim};
use super::super::types::{PendingToolInstall, TOOL_PENDING, TOOL_SCHEMA};
use crate::commands::package_runtime::{parse_request, valid_bin_name};
use anyhow::{Context, Result, bail};
use std::fs;

pub(in crate::commands::tool) fn install(
    request: &str,
    selected_bin: Option<&str>,
    allow_scripts: bool,
) -> Result<()> {
    require_web_lane()?;
    let Some(request) = request.strip_prefix("npm:") else {
        bail!(
            "global CLI tools require an explicit source, for example `npm:@openai/codex`; this release implements npm tools only"
        );
    };
    let (package, selector) = parse_request(request)?;
    if selected_bin.is_some_and(|binary| !valid_bin_name(binary)) {
        return Err(crate::commands::package_runtime::registry_package_invalid_bin());
    }

    let (data_root, tools_root, bin_root) = managed_roots(true)?;
    let project_name = tool_project_name(package);
    let project_root = tools_root.join(&project_name);
    match fs::symlink_metadata(&project_root) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            bail!("managed tool path '{}' is unsafe", project_root.display())
        }
        Ok(_) => {
            if let Some(record) = read_record_if_present(&project_root)? {
                if record.package == package {
                    bail!(
                        "tool project already exists at '{}'; use `mgc tool update {}` or `mgc tool remove {}`",
                        project_root.display(),
                        record.binary,
                        record.binary
                    );
                }
                bail!("managed tool project identity does not match requested package");
            }
            let pending = read_pending_if_present(&project_root)?.ok_or_else(|| {
                anyhow::anyhow!(
                    "unrecognized or interrupted tool data at '{}'; inspect and remove it manually before retrying",
                    project_root.display()
                )
            })?;
            if pending.package != package {
                bail!("interrupted tool package does not match the requested package");
            }
            if let Some(record) = pending.record {
                let shim = shim_path(&bin_root, &record.binary);
                remove_owned_shim(&project_root, &record, &shim)?;
            }
            fs::remove_dir_all(&project_root)
                .context("remove interrupted managed tool installation before retrying")?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    fs::create_dir(&project_root)?;
    set_private_directory(&project_root)?;
    let mut pending = PendingToolInstall {
        schema: TOOL_SCHEMA,
        source: "npm".to_string(),
        package: package.to_string(),
        selector: selector.to_string(),
        project_name: project_name.clone(),
        record: None,
    };
    if let Err(error) = write_pending(&project_root, &pending) {
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }

    let result = install_into_project(
        &project_root,
        &data_root,
        package,
        selector,
        selected_bin,
        allow_scripts,
    );
    let record = match result {
        Ok(record) => record,
        Err(error) => {
            let _ = fs::remove_dir_all(&project_root);
            return Err(error);
        }
    };

    pending.record = Some(record.clone());
    if let Err(error) = write_pending(&project_root, &pending) {
        let shim = shim_path(&bin_root, &record.binary);
        let _ = remove_owned_shim(&project_root, &record, &shim);
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }

    let shim = shim_path(&bin_root, &record.binary);
    if let Err(error) = ensure_shim(&project_root, &record, &shim, false) {
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }
    if let Err(error) = write_record(&project_root, &record) {
        let _ = remove_owned_shim(&project_root, &record, &shim);
        let _ = fs::remove_dir_all(&project_root);
        return Err(error);
    }
    if let Err(error) = fs::remove_file(project_root.join(TOOL_PENDING)) {
        mgc_ui::warning(&format!(
            "tool is installed, but its recovery marker could not be removed: {error}"
        ));
    }
    if let Err(error) = mgc_platform::paths::ensure_global_bin_on_path() {
        let _ = remove_owned_shim(&project_root, &record, &shim);
        let _ = fs::remove_dir_all(&project_root);
        return Err(error).context(
            "tool installed, but user PATH registration failed; installation was rolled back",
        );
    }

    mgc_ui::success(&format!(
        "Installed {}@{} as '{}'. Open a new terminal to use it.",
        record.package, record.installed_version, record.binary
    ));
    Ok(())
}
