use super::{
    allowed_tool_membership::ensure_tool_is_allowed, device_tool_scope::restrict_device_tool_scope,
    forbidden_pm_tool::reject_forbidden_pm_tool,
    language_runtime_scope::restrict_language_runtime_scope,
    non_empty_tool_name::validate_non_empty_tool_name,
    project_execution_scope::is_project_execution_scope,
};
use crate::allowlist::project_local_executable::is_project_local_executable;
use crate::allowlist::{ExecutionScope, normalize_script_token};
use anyhow::Result;
use std::path::Path;

/// Check tool with execution scope — new primary API.
/// PM tools (npm/pnpm/yarn/bun) are forbidden in every scope.
pub fn check_tool_with_scope(
    name: &str,
    scope: ExecutionScope,
    project_root: Option<&Path>,
) -> Result<()> {
    check_tool_with_scope_compat(name, scope, project_root, None)
}

/// Historical API retained for source compatibility. The compatibility
/// argument never authorizes a process; deny-list policy is enforced here,
/// beneath every CLI/router call site.
/// Giữ API cũ để tương thích source; tham số compat không cấp quyền spawn.
pub fn check_tool_with_scope_compat(
    name: &str,
    scope: ExecutionScope,
    project_root: Option<&Path>,
    _compat_runtime: Option<&str>,
) -> Result<()> {
    let name = validate_non_empty_tool_name(name)?;

    let normalized = normalize_script_token(name).unwrap_or_else(|| name.to_ascii_lowercase());

    // Package managers and rival runtimes are forbidden in every scope.
    reject_forbidden_pm_tool(name, &normalized, scope)?;

    // ADB can affect attached devices; require the narrow device-control scope.
    // ADB có thể tác động thiết bị thật; bắt buộc dùng scope điều khiển riêng.
    // xcrun drives simulators/devices; same narrow scope as adb.
    // xcrun điều khiển simulator/thiết bị; scope hẹp như adb.
    restrict_device_tool_scope(&normalized, scope)?;

    // These runtimes execute project code; keep them out of install hooks and
    // device-control calls. `mgc run` supplies ScriptRunner explicitly.
    // (Runtime này chạy code project; không cho install hook/device-control
    // gọi. `mgc run` truyền ScriptRunner tường minh.)
    restrict_language_runtime_scope(name, &normalized, scope)?;

    if is_project_execution_scope(scope)
        && let Some(root) = project_root
        && is_project_local_executable(name, root)
    {
        return Ok(());
    }

    // Non-PM tools: check against general allowlist
    ensure_tool_is_allowed(name, &normalized)
}

/// Apply scope-sensitive device and catalogued-runtime denies without the
/// static host-tool allowlist for an already-resolved project/package binary.
/// Áp deny device và runtime có trong catalog theo scope cho binary
/// project/package đã resolve, nhưng không áp allowlist tĩnh của tool host.
pub(crate) fn check_project_binary_scope(name: &str, scope: ExecutionScope) -> Result<()> {
    let name = validate_non_empty_tool_name(name)?;
    let normalized = normalize_script_token(name).unwrap_or_else(|| name.to_ascii_lowercase());

    restrict_device_tool_scope(&normalized, scope)?;
    restrict_language_runtime_scope(name, &normalized, scope)
}
