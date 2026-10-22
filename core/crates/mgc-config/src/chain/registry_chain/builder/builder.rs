//! Build ordered registry candidates and compare registry URLs.
//! Tạo candidate registry theo thứ tự và so sánh URL registry.

use crate::registry::Registry;

pub const DEFAULT_NPM_REGISTRY: &str = "https://registry.npmjs.org";
pub const WEB_REGISTRY_URL_ENV: &str = "MAGICORE_WEB_REGISTRY_URL";

pub(in crate::chain::owners::registry_chain) fn registry_chain_with_npmrc(
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

#[cfg(test)]
#[path = "test/builder.rs"]
mod tests;
