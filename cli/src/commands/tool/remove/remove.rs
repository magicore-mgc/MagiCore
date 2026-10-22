use super::super::managed_roots::{managed_roots, shim_path};
use super::super::record_lookup::find_record_by_binary;
use super::super::shim::remove_owned_shim;
use crate::commands::package_runtime::valid_bin_name;
use anyhow::{Result, bail};
use std::fs;

pub(in crate::commands::tool) fn remove(binary: &str) -> Result<()> {
    if !valid_bin_name(binary) {
        return Err(crate::commands::package_runtime::registry_package_invalid_bin());
    }
    let (_data_root, tools_root, bin_root) = managed_roots(false)?;
    let Some((project_root, record)) = find_record_by_binary(&tools_root, binary)? else {
        bail!("managed tool '{}' is not installed", binary);
    };
    let shim = shim_path(&bin_root, binary);
    remove_owned_shim(&project_root, &record, &shim)?;
    fs::remove_dir_all(&project_root)?;
    mgc_ui::success(&format!("Removed {} ({})", record.binary, record.package));
    Ok(())
}
