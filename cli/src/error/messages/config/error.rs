//! Project configuration and identity diagnostics.
//! Lỗi cấu hình và danh tính project.

use anyhow::Error;
use anyhow::anyhow;

/// A project TOML update could not obtain the shared project writer lock.
/// Không lấy được khóa project để cập nhật mgc.toml.
pub fn config_project_lock_failed(root: &std::path::Path, reason: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "cannot update project configuration in '{}' while project state is changing or unrecovered: {reason}",
        root.display()
    )
}

pub fn config_key_missing(key: &str) -> Error {
    anyhow!("config key '{key}' not found (checked env, project .npmrc, and ~/.npmrc)")
}

pub fn config_core_identity_managed_by_signature() -> Error {
    anyhow!(
        "the project core identity is immutable through `mgc config`; `mgc init --signature <core>` writes a plain-text marker and cannot reassign an existing project"
    )
}

/// Refuse a per-core command when another core owns the project directory.
/// Từ chối lệnh core nếu project do core khác sở hữu.
pub fn project_core_identity_mismatch(
    project_root: &std::path::Path,
    expected: &str,
    detected: &str,
) -> Error {
    anyhow!(
        "project '{}' belongs to core '{}'; refusing to run the '{}' core command",
        project_root.display(),
        detected,
        expected,
    )
}

/// Refuse to route a per-core command when project identity cannot be proven.
/// Không định tuyến lệnh core khi chưa xác minh được identity project.
pub fn project_core_identity_missing(project_root: &std::path::Path, expected: &str) -> Error {
    anyhow!(
        "cannot prove project core identity for '{}'; refusing to run the '{}' core command",
        project_root.display(),
        expected,
    )
}

pub fn core_selector_conflicts_command(command_core: &str, selected_core: &str) -> Error {
    anyhow!(
        "global --core '{}' conflicts with the explicit '{}' core command",
        selected_core,
        command_core,
    )
}

pub fn config_project_file_symlink(path: &std::path::Path) -> Error {
    anyhow!("project config '{}' must not be a symlink", path.display())
}

pub fn config_project_file_not_regular(path: &std::path::Path) -> Error {
    anyhow!("project config '{}' must be a regular file", path.display())
}

pub fn config_project_file_invalid_path() -> Error {
    anyhow!("configuration path has no file name")
}
