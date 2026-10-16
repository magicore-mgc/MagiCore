//! Registry chain hợp nhất (ITEM 2): một nguồn sự thật cho thứ tự registry
//! — env override → mgc.toml [[registries]] (priority) → .npmrc registry= → npmjs.

use crate::registry::Registry;

pub const DEFAULT_NPM_REGISTRY: &str = "https://registry.npmjs.org";
pub const WEB_REGISTRY_URL_ENV: &str = "MAGICORE_WEB_REGISTRY_URL";

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

/// Build a validated chain and resolve missing credentials by URL scope.
/// Không nuốt lỗi npmrc hoặc scope mâu thuẫn; token env/config tường minh được ưu tiên.
pub fn registry_chain_checked(
    project_root: Option<&std::path::Path>,
    project_config: Option<&crate::project::ProjectConfig>,
) -> anyhow::Result<Vec<Registry>> {
    Ok(registry_chain_scoped_checked(project_root, project_config)?
        .into_iter()
        .map(|entry| entry.registry)
        .collect())
}

/// Registry and credential scope travel together; tokens are never Debug output.
/// Native transport phải giữ scope tới request; không dùng API value-only để wiring.
pub struct ScopedRegistry {
    pub registry: Registry,
    pub credential_scope: Option<url::Url>,
}

/// Resolve explicit origin credentials and npmrc path credentials without flattening scope.
/// Token env/config giữ contract origin; npmrc giữ scope cụ thể đã chọn.
pub fn registry_chain_scoped_checked(
    project_root: Option<&std::path::Path>,
    project_config: Option<&crate::project::ProjectConfig>,
) -> anyhow::Result<Vec<ScopedRegistry>> {
    let npmrc = project_root.map(crate::npmrc::NpmRc::load).transpose()?;
    let chain = registry_chain_with_npmrc(project_config, npmrc.as_ref(), false);
    let validation = crate::npmrc::NpmRc::default();
    let mut scoped = Vec::with_capacity(chain.len());
    for mut registry in chain {
        validation.credential_for_url_checked(&registry.url)?;
        let credential_scope = if registry.token.is_some() {
            let mut origin = url::Url::parse(&registry.url)?;
            origin.set_path("/");
            origin.set_query(None);
            origin.set_fragment(None);
            Some(origin)
        } else if let Some(credential) = npmrc
            .as_ref()
            .map(|rc| rc.credential_for_url_checked(&registry.url))
            .transpose()?
            .flatten()
        {
            registry.token = Some(credential.token.clone());
            Some(credential.scope)
        } else {
            None
        };
        scoped.push(ScopedRegistry {
            registry,
            credential_scope,
        });
    }
    Ok(scoped)
}

fn registry_chain_with_npmrc(
    project_config: Option<&crate::project::ProjectConfig>,
    npmrc: Option<&crate::npmrc::NpmRc>,
    legacy_behavior: bool,
) -> Vec<Registry> {
    if let Some(env_url) = std::env::var(WEB_REGISTRY_URL_ENV)
        .ok()
        .filter(|v| !v.is_empty())
    {
        let mut r = Registry::new("env".to_string(), env_url);
        r.priority = 0;
        r.token = std::env::var("MAGICORE_WEB_REGISTRY_TOKEN").ok();
        return vec![r];
    }

    let mut chain: Vec<Registry> = Vec::new();

    if let Some(cfg) = project_config {
        let mut from_toml = cfg.registries.clone();
        from_toml.sort_by_key(|r| r.priority);
        for reg in from_toml {
            if !chain
                .iter()
                .any(|c| same_registry_url(&c.url, &reg.url, legacy_behavior))
            {
                chain.push(reg);
            }
        }
    }

    // let-chain edition 2024 — gộp điều kiện lồng nhau theo clippy 1.98.
    if let Some(npmrc) = npmrc
        && let Some(url) = npmrc.registry_for(None)
        && !chain
            .iter()
            .any(|c| same_registry_url(&c.url, &url, legacy_behavior))
    {
        let mut r = Registry::new("npmrc".to_string(), url.clone());
        r.priority = u32::MAX;
        if legacy_behavior {
            r.token = npmrc.token_for(&host_of(&url)).cloned();
        }
        chain.push(r);
    }

    if !chain
        .iter()
        .any(|c| same_registry_url(&c.url, DEFAULT_NPM_REGISTRY, legacy_behavior))
    {
        let mut r = Registry::new("npmjs".to_string(), DEFAULT_NPM_REGISTRY.to_string());
        r.priority = u32::MAX;
        chain.push(r);
    }

    chain
}

fn same_url(a: &str, b: &str) -> bool {
    a.trim_end_matches('/')
        .eq_ignore_ascii_case(b.trim_end_matches('/'))
}

fn host_of(url: &str) -> String {
    url::Url::parse(url)
        .map(|u| u.host_str().unwrap_or("").to_string())
        .unwrap_or_default()
}

// Canonicalize authority while keeping case-sensitive registry paths distinct.
// Chỉ chuẩn hóa scheme/host/port; không gộp hai registry có path khác hoa/thường.
fn same_registry_url(a: &str, b: &str, legacy_behavior: bool) -> bool {
    if legacy_behavior {
        return same_url(a, b);
    }
    let (Ok(a), Ok(b)) = (url::Url::parse(a), url::Url::parse(b)) else {
        return false;
    };
    a.scheme() == b.scheme()
        && a.host_str() == b.host_str()
        && a.port_or_known_default() == b.port_or_known_default()
        && a.username() == b.username()
        && a.password() == b.password()
        && a.path().trim_end_matches('/') == b.path().trim_end_matches('/')
        && a.query() == b.query()
        && a.fragment() == b.fragment()
}
