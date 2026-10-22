//! Best-effort compatibility entry point for registry-chain resolution.
//! Entry point tương thích best-effort cho việc resolve registry-chain.

use crate::registry::Registry;

use super::super::builder::{WEB_REGISTRY_URL_ENV, registry_chain_with_npmrc};

/// Chain registry theo thứ tự ưu tiên (entry 0 = primary).
/// env MAGICORE_WEB_REGISTRY_URL set → CHỈ registry đó (override — không thêm
/// fallback, giữ semantic cũ: test/CI pin 1 registry).
/// Ngược lại: mgc.toml [[registries]] (priority asc) → .npmrc registry= → npmjs
/// (dedupe theo URL).
pub fn registry_chain(
    project_root: Option<&std::path::Path>,
    project_config: Option<&crate::project::ProjectConfig>,
) -> Vec<Registry> {
    // Preserve the legacy explicit-env fast path; checked callers use the API below.
    // Giữ fast path env cũ; caller native cần kiểm lỗi dùng API checked.
    let env_override = std::env::var(WEB_REGISTRY_URL_ENV)
        .ok()
        .is_some_and(|value| !value.is_empty());
    let npmrc = if env_override {
        None
    } else {
        project_root.and_then(|root| crate::npmrc::NpmRc::load(root).ok())
    };
    registry_chain_with_npmrc(project_config, npmrc.as_ref(), true)
}
