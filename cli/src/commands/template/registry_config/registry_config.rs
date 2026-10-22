use anyhow::Result;
use mgc_config::npmrc::NpmRc;
use mgc_publish::auth::resolve_auth;
use std::path::Path;

pub(super) const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";

pub(in crate::commands::template) const TEMPLATE_FETCH_TIMEOUT_SECS: u64 = 30;

pub(in crate::commands::template) const MAX_TEMPLATE_REDIRECTS: usize = 5;

pub(in crate::commands::template) const MAX_TEMPLATE_METADATA_BYTES: usize = 16 * 1024 * 1024;

pub(in crate::commands::template) const MAX_TEMPLATE_TARBALL_BYTES: usize = 64 * 1024 * 1024;

pub(in crate::commands::template) const MAX_TEMPLATE_VERSION_BYTES: u64 = 1024;

/// Resolve registry: --registry flag → MGC_NPM_REGISTRY env → .npmrc → default.
pub(in crate::commands::template) fn select_registry(flag: Option<&str>) -> Result<String> {
    if let Some(url) = flag {
        return Ok(url.to_string());
    }
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(url) = std::env::var("MGC_NPM_REGISTRY")
        && !url.is_empty()
    {
        return Ok(url);
    }
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(npmrc) = NpmRc::load(Path::new("."))
        && let Some(url) = npmrc.registry_for(None)
    {
        return Ok(url);
    }
    Ok(DEFAULT_REGISTRY.to_string())
}

/// Auth resolution cho fetch — registry công khai không cần; registry private
/// (token admin) yêu cầu header, nên lấy auth nếu cấu hình (npmrc/env).
pub(in crate::commands::template) async fn register_fetch_auth(
    registry: &str,
) -> mgc_publish::auth::Auth {
    let npmrc = NpmRc::load(Path::new("."));
    let npmrc = match npmrc {
        Ok(n) => n,
        Err(_) => return mgc_publish::auth::Auth::default(),
    };
    resolve_auth(&npmrc, registry, None, None)
        .unwrap_or_else(|_| mgc_publish::auth::Auth::default())
}
