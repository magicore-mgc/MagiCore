//! Selects the write-target lockfile schema from project configuration.
//! Chọn schema lockfile mục tiêu ghi từ cấu hình project.

use super::super::ProjectConfig;

/// Write-target lockfile schema for new locks, shared by every adapter
/// write path (one rule, no per-lane drift): v4 only with explicit
/// `mgc.toml [lock] schema = "4"` (the documented `"v4"` spelling also
/// works); absent means v3; anything else fails closed.
/// (Schema lockfile đích ghi cho lock mới, dùng chung mọi đường ghi
/// adapter (một luật, không lệch lane): chỉ v4 khi tường minh;
/// vắng nghĩa là v3; còn lại fail-closed.)
pub fn lock_schema_is_v4(project_root: &std::path::Path) -> anyhow::Result<bool> {
    let schema = ProjectConfig::load(project_root)?
        .and_then(|config| config.lock)
        .and_then(|lock| lock.schema)
        .unwrap_or_else(|| "3".to_string());
    let normalized = schema.strip_prefix('v').unwrap_or(&schema);
    match normalized {
        "4" => Ok(true),
        "3" => Ok(false),
        _ => anyhow::bail!(
            "mgc.toml [lock] schema = \"{schema}\" is not supported (expected \"3\" or \"4\")"
        ),
    }
}
