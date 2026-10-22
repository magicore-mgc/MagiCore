// Workspace/core identity checks — resolve project ownership and reject contradictory selectors.
// Owner này đọc identity dự án và từ chối selector core mâu thuẫn.
use super::super::command_name::command_name;
use crate::Commands;
use anyhow::{Result, bail};
use std::path::Path;

/// Read a workspace manifest name for `--filter`; missing manifests return `None`.
/// Đọc tên từ manifest workspace cho `--filter`; thiếu manifest thì trả `None`.
pub(in crate::dispatch::engine) fn workspace_package_name(
    project_root: &Path,
) -> Result<Option<String>> {
    Ok(mgc_workspace::read_package_manifest(project_root)?.map(|manifest| manifest.name))
}

/// Resolve a recursive workspace's core without allowing a global selector to
/// contradict its persisted MGC identity.
/// Xác định core workspace đệ quy, không cho cờ toàn cục trái identity đã lưu.
pub(in crate::dispatch::engine) fn resolve_recursive_workspace_core(
    workspace: &Path,
    requested: Option<&str>,
) -> Result<Option<String>> {
    let marker = mgc_config::project::ProjectConfig::read_core_marker(workspace)?;
    let Some(requested) = requested else {
        return Ok(marker);
    };
    // Marker reading already cross-checks mgc.toml; only load config separately
    // for a legacy workspace that has config but no marker.
    // (Marker reader đã đối chiếu mgc.toml; chỉ load riêng project legacy thiếu marker.)
    let persisted = match marker.as_deref() {
        Some(core) => Some(core.to_string()),
        None => mgc_config::project::ProjectConfig::load(workspace)?.map(|config| config.ecosystem),
    };

    let requested_core = mgc_types::Ecosystem::from_str(requested)
        .ok_or_else(|| crate::error::unknown_ecosystem(requested))?;
    if let Some(persisted) = persisted {
        let persisted_core = mgc_types::Ecosystem::from_str(&persisted)
            .ok_or_else(|| crate::error::unknown_ecosystem(&persisted))?;
        if requested_core != persisted_core {
            bail!(
                "recursive --core '{}' conflicts with workspace '{}' identity '{}'; refusing cross-core dispatch",
                requested_core.as_str(),
                workspace.display(),
                persisted_core.as_str(),
            );
        }
    }
    Ok(Some(requested_core.as_str().to_string()))
}

/// Return the core selected by a core-specific dependency command.
/// Lấy core được chọn bởi lệnh dependency định danh sẵn.
pub(in crate::dispatch::engine) fn explicit_dependency_core(
    command: &Commands,
) -> Option<&'static str> {
    let (operation, core) = command_name(command).split_once('-')?;
    matches!(operation, "install" | "add" | "remove" | "update" | "list")
        .then(|| mgc_types::Ecosystem::from_str(core).map(|ecosystem| ecosystem.as_str()))
        .flatten()
}

/// Refuse a core-specific dependency command that contradicts either the
/// global selector or the workspace's persisted/detected owner.
/// Từ chối lệnh dependency trái cờ toàn cục hoặc owner đã lưu của workspace.
pub(in crate::dispatch::engine) fn validate_explicit_dependency_core(
    workspace: &Path,
    command_core: &str,
    global_core: Option<&str>,
) -> Result<()> {
    let command_core = mgc_types::Ecosystem::from_str(command_core)
        .ok_or_else(|| crate::error::unknown_ecosystem(command_core))?;
    if let Some(global_core) = global_core {
        let global_core = mgc_types::Ecosystem::from_str(global_core)
            .ok_or_else(|| crate::error::unknown_ecosystem(global_core))?;
        if command_core != global_core {
            return Err(crate::error::core_selector_conflicts_command(
                command_core.as_str(),
                global_core.as_str(),
            ));
        }
    }

    let Some(actual) = mgc_config::project::ProjectConfig::detect_core(workspace)? else {
        return Err(crate::error::project_core_identity_missing(
            workspace,
            command_core.as_str(),
        ));
    };
    let actual = mgc_types::Ecosystem::from_str(&actual)
        .ok_or_else(|| crate::error::unknown_ecosystem(&actual))?;
    if command_core != actual {
        return Err(crate::error::project_core_identity_mismatch(
            workspace,
            command_core.as_str(),
            actual.as_str(),
        ));
    }
    Ok(())
}
