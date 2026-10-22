//! Resolve the policy lane tuple and lock-bound profile.
//! Giải quyết bộ lane policy và profile gắn với lock.

use std::path::Path;

/// Resolve a lane context from locked web framework metadata first, then
/// from project configuration. An unknown version remains empty and cannot
/// match a qualified profile or exception.
/// (Ưu tiên framework web đã khóa, rồi dùng cấu hình project. Version chưa
/// biết để rỗng nên không khớp profile hay exception đã qualify.)
pub(in crate::commands::policy) fn lane_context(
    project_root: &Path,
    operation: &str,
) -> Option<mgc_exec::policy::PolicyContext> {
    let config = mgc_config::project::ProjectConfig::load(project_root)
        .ok()
        .flatten()?;
    let (framework, version) = match crate::commands::freshness::resolve_web_framework(project_root)
    {
        Ok(Some((framework, version))) => (framework, version),
        Ok(None) => (
            config
                .frameworks
                .first()
                .cloned()
                .unwrap_or_else(|| config.ecosystem.clone()),
            String::new(),
        ),
        Err(_) => return None,
    };
    Some(mgc_exec::policy::PolicyContext {
        framework,
        version,
        os: std::env::consts::OS.to_string(),
        operation: operation.to_string(),
    })
}

/// Load only a profile bound to the exact lane tuple and current lock bytes.
/// Invalid or stale profile data cannot enable a manifest exception.
/// Chỉ nạp profile gắn đúng lane tuple và bytes lock hiện tại. Profile sai
/// hoặc hết hạn không thể mở manifest exception.
pub(in crate::commands::policy) fn active_profile(
    project_root: &Path,
    context: &mgc_exec::policy::PolicyContext,
) -> Option<mgc_exec::policy::LaneProfile> {
    let lock_bytes = std::fs::read(project_root.join("mgc.lock")).ok()?;
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&lock_bytes).to_hex();
    match mgc_exec::spawn_profile::find_active_profile(
        &mgc_platform::paths::mgc_data_dir().join("profiles"),
        &context.framework,
        &context.version,
        &context.os,
        &context.operation,
        Some(&digest),
    ) {
        Ok(profile) => profile,
        Err(error) => {
            mgc_ui::warning(&format!(
                "ignoring invalid spawn profile for policy exception lookup: {error:?}"
            ));
            None
        }
    }
}

#[cfg(test)]
#[path = "test/context.rs"]
mod tests;
