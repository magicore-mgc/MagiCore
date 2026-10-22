//! Script, hook, tool, and runtime policy diagnostics.
//! Lỗi script, hook, tool và runtime policy.

use anyhow::Error;
use anyhow::anyhow;

pub fn tool_not_installed_project(tool: &str) -> Error {
    anyhow!(
        "{tool} is not installed in the project — run `mgc install` first (tools resolve from node_modules/.bin/{tool})"
    )
}

pub fn project_tool_failed(tool: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "{tool} failed: {e} — run `mgc install` in this project first (tools install via node_modules/.bin)"
    )
}

pub fn script_not_found(script: &str) -> Error {
    anyhow!(
        "Script '{script}' not found. Define it in 'mgc.toml' under [scripts] or in 'package.json'."
    )
}

pub fn forbidden_pm_script(cmd: &str, manifest: &std::path::Path, pm: &str) -> Error {
    anyhow!(
        "Script '{cmd}' in '{}' delegates to forbidden package manager '{pm}'. Core-web task execution must not bounce through another package manager.",
        manifest.display()
    )
}

pub fn unsupported_script(script: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("Unsupported script '{script}': {e}")
}

/// Publish lifecycle hooks require explicit, one-command approval.
/// (Hook publish cần được cho phép tường minh cho lần chạy này.)
pub fn publish_lifecycle_opt_in_required() -> Error {
    anyhow!(
        "package defines publish lifecycle hooks; review them and rerun with --allow-scripts, or use --ignore-scripts to skip them"
    )
}

/// Malformed publish lifecycle metadata must never look like an empty hook list.
/// (Metadata lifecycle sai không được giả như danh sách hook rỗng.)
pub fn publish_lifecycle_policy_invalid() -> Error {
    anyhow!("package.json publish lifecycle scripts must be an object of string commands")
}

pub fn hook_failed(event: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("hook {event} failed: {e}")
}

pub fn dir_missing(dir: &str, cause: String) -> Error {
    anyhow!("cannot change to directory '{dir}': {cause}")
}

pub fn runtime_dangerous_flag_rejected(runtime: &str, flag: &str) -> Error {
    anyhow!("Rejected dangerous {runtime} flag: {flag}. Use project scripts only.")
}

pub fn runtime_dangerous_permission_rejected(runtime: &str, permission: &str) -> Error {
    anyhow!(
        "Rejected dangerous {runtime} permission: {permission}. Explicitly allow in deno.json tasks if needed."
    )
}

/// Refuse native dev when a stored spawn profile exists but cannot be read.
/// (Từ chối native dev khi spawn profile đã lưu tồn tại nhưng không đọc được.)
pub fn invalid_spawn_profile(detail: &str) -> Error {
    anyhow!("invalid spawn profile; refusing native dev pre-flight: {detail}")
}
