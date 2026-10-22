use super::super::managed_roots::tool_project_name;
use super::super::types::{
    PendingToolInstall, TOOL_MANIFEST, TOOL_PENDING, TOOL_SCHEMA, ToolRecord,
};
use crate::commands::package_runtime::{parse_request, valid_bin_name};
use anyhow::{Context, Result, bail};
use std::path::Path;

pub(in crate::commands::tool) fn read_record_if_present(root: &Path) -> Result<Option<ToolRecord>> {
    let path = root.join(TOOL_MANIFEST);
    let Some(content) =
        mgc_config::project::read_regular_project_text(&path, "managed tool record")?
    else {
        return Ok(None);
    };
    let record: ToolRecord = serde_json::from_str(&content)
        .with_context(|| format!("managed tool record '{}' is malformed", path.display()))?;
    validate_tool_record(root, &record)?;
    Ok(Some(record))
}
pub(in crate::commands::tool) fn read_pending_if_present(
    root: &Path,
) -> Result<Option<PendingToolInstall>> {
    let path = root.join(TOOL_PENDING);
    let Some(content) =
        mgc_config::project::read_regular_project_text(&path, "managed tool recovery marker")?
    else {
        return Ok(None);
    };
    let pending: PendingToolInstall = serde_json::from_str(&content).with_context(|| {
        format!(
            "managed tool recovery marker '{}' is malformed",
            path.display()
        )
    })?;
    let request = format!("{}@{}", pending.package, pending.selector);
    let identity_valid = pending.schema == TOOL_SCHEMA
        && pending.source == "npm"
        && parse_request(&request).is_ok_and(|(package, _)| package == pending.package)
        && pending.project_name == tool_project_name(&pending.package)
        && root.file_name().and_then(|name| name.to_str()) == Some(&pending.project_name);
    if !identity_valid {
        bail!(
            "managed tool recovery marker '{}' has an unsupported schema or identity",
            path.display()
        );
    }
    if let Some(record) = &pending.record {
        validate_tool_record(root, record)?;
        if record.source != pending.source
            || record.package != pending.package
            || record.selector != pending.selector
            || record.project_name != pending.project_name
        {
            bail!(
                "managed tool recovery marker '{}' does not match its install record",
                path.display()
            );
        }
    }
    Ok(Some(pending))
}
fn validate_tool_record(root: &Path, record: &ToolRecord) -> Result<()> {
    let package_request = format!("{}@{}", record.package, record.selector);
    let package_valid =
        parse_request(&package_request).is_ok_and(|(package, _)| package == record.package);
    let target_valid = normalize_tool_target(Path::new(&record.target)).is_ok();
    if record.schema != TOOL_SCHEMA
        || record.source != "npm"
        || !valid_bin_name(&record.binary)
        || !package_valid
        || !target_valid
        || record.project_name != tool_project_name(&record.package)
        || root.file_name().and_then(|name| name.to_str()) != Some(&record.project_name)
    {
        bail!(
            "managed tool data at '{}' has an unsupported schema or identity",
            root.display()
        );
    }
    Ok(())
}
pub(in crate::commands::tool) fn normalize_tool_target(target: &Path) -> Result<String> {
    // npm bin paths are portable manifest values; normalize both separator
    // styles before validating so records behave identically across OSes.
    // Đường dẫn bin của npm là dữ liệu manifest đa nền tảng; chuẩn hóa hai
    // kiểu dấu phân cách trước khi kiểm tra để record nhất quán mọi OS.
    let portable = target.to_string_lossy().replace('\\', "/");
    if portable.is_empty() || portable.starts_with('/') {
        return Err(crate::commands::package_runtime::registry_package_invalid_bins());
    }
    let mut components = Vec::new();
    for component in portable.split('/') {
        match component {
            "" | "." => {}
            ".." => return Err(crate::commands::package_runtime::registry_package_invalid_bins()),
            component if component.contains(':') || component.chars().any(char::is_control) => {
                return Err(crate::commands::package_runtime::registry_package_invalid_bins());
            }
            component => components.push(component),
        }
    }
    if components.is_empty() {
        return Err(crate::commands::package_runtime::registry_package_invalid_bins());
    }
    Ok(components.join("/"))
}
