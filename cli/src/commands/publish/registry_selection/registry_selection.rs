use super::super::publish::PublishArgs;
use anyhow::{Result, bail};
use mgc_config::{npmrc::NpmRc, project::ProjectConfig};
use std::path::Path;

/// Select registry per 01 §2 priority
pub(in crate::commands) fn select_registry(
    pkg_json: &serde_json::Value,
    project_root: &Path,
    project: &ProjectConfig,
    args: &PublishArgs,
) -> Result<String> {
    // 1. --registry flag
    if let Some(r) = &args.registry {
        return Ok(r.clone());
    }

    // 2. publishConfig.registry in package.json
    if let Some(url) = pkg_json
        .get("publishConfig")
        .and_then(|p| p.get("registry"))
        .and_then(|v| v.as_str())
    {
        return Ok(url.to_string());
    }

    // 3. .npmrc registry / scope registry
    let npmrc = NpmRc::load(project_root)?;
    let scope = pkg_json
        .get("name")
        .and_then(|v| v.as_str())
        .and_then(|n| n.strip_prefix('@').map(|s| format!("@{}", s)));
    if let Some(url) = npmrc.registry_for(scope.as_deref()) {
        return Ok(url);
    }

    // 4. mgc.toml [registry]
    if let Some(reg) = project.registries.first() {
        return Ok(reg.url.clone());
    }

    // 5. Default
    Ok(DEFAULT_REGISTRY.to_string())
}

/// Verify token with whoami endpoint (npmjs)
pub(in crate::commands) async fn verify_token(
    registry_url: &str,
    auth: &mgc_publish::auth::Auth,
) -> Result<()> {
    let client = reqwest::Client::new();
    let whoami_url = format!("{}/-/whoami", registry_url.trim_end_matches('/'));
    let req = client.get(&whoami_url);
    let req = if let Some(h) = auth.header_value() {
        req.header("Authorization", h)
    } else {
        req
    };
    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() {
        bail!("Token invalid (whoami failed: {})", status);
    }
    Ok(())
}

const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";
