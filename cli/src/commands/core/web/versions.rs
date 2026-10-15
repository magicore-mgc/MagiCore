//! `web/versions.rs` — Web scaffold npm version resolution (P1 split
//! from web.rs, Tech Lead 2026-09-11). One responsibility: resolve the
//! pinned version for a scaffold package — active age policy first, then
//! hermetic env override, live npm registry, and embedded baseline fallback.
//! Age-gated resolution never falls back to an unverified version.
//! `web/versions.rs` — Giải quyết phiên bản npm cho web scaffold (tách
//! từ web.rs theo P1). Policy tuổi phát hành được áp dụng trước; đường
//! age-gated không fallback sang phiên bản chưa xác minh.

use std::collections::HashMap;
use std::path::Path;

#[cfg(feature = "web")]
use anyhow::Context;
use anyhow::Result;
#[cfg(feature = "web")]
use mgc_types::{PackageName, Version, VersionRange};
#[cfg(feature = "web")]
use mgc_web_adapter::native::npm_registry::{NpmRegistry, PackageMetadata};
#[cfg(feature = "web")]
use mgc_web_adapter::provider::{AgePolicy, eligible_versions};

/// Default registry — overridable via config elsewhere; this module
/// only READS from it (RULE §12: named constant, not inline literal).
/// Registry mặc định — module chỉ ĐỌC (RULE §12: hằng số có tên).
pub(crate) const DEFAULT_NPM_REGISTRY: &str = "https://registry.npmjs.org";

/// Env override list ("pkg@^1,pkg2@^2") for hermetic CI scaffolds without an age gate.
/// Danh sách env ghi đè version cho CI hermetic khi age gate không bật.
const SCAFFOLD_VERSION_OVERRIDES_ENV: &str = "MAGICORE_WEB_SCAFFOLD_VERSION_OVERRIDES";

/// Version baseline — trước đây ở templates/web/versions/scaffold-baseline.toml;
/// registry-first (templates/ xóa khỏi repo) → giữ thẳng trong code.
const SCAFFOLD_BASELINE_VERSIONS_TOML: &str = r#"[versions]
vue = "^3.5.44"
react = "^19.3.1"
react-dom = "^19.3.1"
vite = "^8.3.4"
"@vitejs/plugin-react" = "^6.1.2"
"@vitejs/plugin-vue" = "^6.0.10"
solid-js = "^1.9.17"
vite-plugin-solid = "^2.11.14"
typescript = "^7.0.2"
"@types/react" = "^19.3.0"
"@types/react-dom" = "^19.3.0"
"@types/node" = "^26.6.5"
tailwindcss = "^4.3.3"
"@sveltejs/kit" = "^3.1.0"
"@sveltejs/vite-plugin-svelte" = "^7.3.1"
"@sveltejs/adapter-auto" = "^8.0.0"
svelte = "^5.57.2"
next = "^16.4.1"
nuxt = "^4.6.1"
unplugin = "^3.4.0"
"@angular/core" = "^22.2.3"
"@angular/platform-browser" = "^22.2.3"
"@angular/platform-browser-dynamic" = "^22.2.3"
"@angular/router" = "^22.2.3"
"@angular/compiler" = "^22.2.3"
"@angular/common" = "^22.2.3"
rxjs = "^7.8.2"
"zone.js" = "^0.16.3"
tslib = "^2.8.1"
"@angular/cli" = "^22.2.2"
"@angular/compiler-cli" = "^22.2.3"
"@angular-devkit/build-angular" = "^22.2.2"
"@builder.io/qwik" = "^1.20.2"
"@builder.io/qwik-city" = "^1.20.2"
astro = "^7.3.8"
express = "^5.3.0"
"@types/express" = "^5.0.6"
hono = "^4.13.13"
"@hono/node-server" = "^2.1.5"
"@nestjs/core" = "^12.1.3"
"@nestjs/common" = "^12.1.3"
"@nestjs/platform-express" = "^12.1.3"
reflect-metadata = "^0.2.2"
zod = "^4.6.7"
"@trpc/server" = "^11.19.0"
fastify = "^5.12.7"
tsx = "^4.23.15"
"@prisma/client" = "^7.10.0"
prisma = "^7.10.0"
vitest = "^5.0.4"
eslint = "^10.12.0"
eslint-config-next = "^16.4.1"
prettier = "^3.9.10"
"@tailwindcss/postcss" = "^4.3.3"
pg = "^8.23.1"
zustand = "^5.0.15"
"@tanstack/react-query" = "^5.104.2"
next-auth = "^4.24.15"
"@playwright/test" = "^1.64.1"
husky = "^9.1.7"
lint-staged = "^17.6.0"
"@biomejs/biome" = "^2.5.15"
clsx = "^2.1.1"
tailwind-merge = "^3.7.0"
class-variance-authority = "^0.7.1"
sass = "^1.105.1"
unocss = "^66.11.0"
daisyui = "^5.7.50"
"@reduxjs/toolkit" = "^2.13.0"
react-redux = "^9.3.0"
jest = "^30.5.2"
"@testing-library/react" = "^16.3.3"
"@testing-library/jest-dom" = "^7.0.1"
jest-environment-jsdom = "^30.5.2"
cypress = "^16.2.0"
drizzle-orm = "^0.45.4"
drizzle-kit = "^0.31.11"
"@clerk/nextjs" = "^7.9.15"
styled-components = "^6.6.0"
"@types/styled-components" = "^5.1.36"
"@commitlint/cli" = "^21.2.3"
"@commitlint/config-conventional" = "^21.2.3"
"@apollo/server" = "^5.5.1"
"@as-integrations/next" = "^4.1.0"
"@trpc/client" = "^11.19.0"
"@trpc/next" = "^11.19.0"
"@grpc/grpc-js" = "^1.14.6"
"@grpc/proto-loader" = "^0.8.1"
lucia = "^3.2.2"
"@lucia-auth/adapter-drizzle" = "^1.1.0"
jose = "^6.2.12"
dotenv-cli = "^11.0.0"
next-i18next = "^16.3.1"
next-pwa = "^5.6.0"
"@storybook/nextjs" = "^10.6.2"
"@storybook/react" = "^10.6.2"
"@sentry/nextjs" = "^11.7.0"
"@vercel/analytics" = "^2.0.1"
"@railway/cli" = "^5.68.0"
flyctl = "^1.2.11"
"#;

/// TOML shape of the baseline table.
/// Kết cấu TOML của bảng baseline.
#[derive(serde::Deserialize)]
struct ScaffoldBaselineVersions {
    versions: HashMap<String, String>,
}

/// Web scaffold's normalized age gate, independent of optional adapter crates.
/// Age gate scaffold Web đã chuẩn hóa, không phụ thuộc adapter optional.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScaffoldAgePolicy {
    pub cutoff_hours: u64,
    pub allow_missing_time: bool,
}

#[cfg(feature = "web")]
impl From<ScaffoldAgePolicy> for AgePolicy {
    fn from(policy: ScaffoldAgePolicy) -> Self {
        Self {
            cutoff_hours: policy.cutoff_hours,
            allow_missing_time: policy.allow_missing_time,
        }
    }
}

/// Resolve scaffold versions through the same active age policy as installation.
/// Giải quyết version scaffold theo cùng age policy đang áp dụng lúc cài đặt.
#[cfg(feature = "web")]
pub(crate) fn scaffold_age_policy(project_root: &Path) -> Result<Option<ScaffoldAgePolicy>> {
    let security = mgc_config::project::load_security_config(project_root).map_err(|error| {
        crate::error::web_scaffold_security_config_unreadable(project_root, &error)
    })?;
    let allow_missing_time = security
        .as_ref()
        .and_then(|config| config.allow_missing_time)
        .unwrap_or(false);
    if !env_truthy("MAGICORE_ALLOW_UNTRUSTED") {
        if let Some(cutoff_hours) = security
            .as_ref()
            .and_then(|config| config.min_age_for_ecosystem("web"))
        {
            return Ok(Some(ScaffoldAgePolicy {
                cutoff_hours,
                allow_missing_time,
            }));
        }
        if let Some(min_age_secs) = store_release_policy(project_root)? {
            return Ok(Some(ScaffoldAgePolicy {
                cutoff_hours: min_age_secs.div_ceil(60 * 60),
                allow_missing_time,
            }));
        }
        if env_truthy("MAGICORE_SECURITY_24H_BLOCK") || env_truthy("MGC_AUDIT_STRICT") {
            return Ok(Some(ScaffoldAgePolicy {
                cutoff_hours: mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS,
                allow_missing_time,
            }));
        }
    }
    Ok(None)
}

#[cfg(not(feature = "web"))]
pub(crate) fn scaffold_age_policy(_project_root: &Path) -> Result<Option<ScaffoldAgePolicy>> {
    Ok(None)
}

#[cfg(feature = "web")]
pub(super) fn store_release_policy(project_root: &Path) -> Result<Option<u64>> {
    let layout = mgc_store::Layout::new(mgc_web_adapter::lockfile::project_cache_dir(project_root));
    let database_path = layout.db_path();
    match std::fs::symlink_metadata(&database_path) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(_) => {
            return Err(crate::error::web_scaffold_store_policy_path_invalid(
                &database_path,
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(crate::error::web_scaffold_store_policy_unreadable(
                &database_path,
                &error,
            ));
        }
    }
    let database = mgc_store::Database::open(&database_path).map_err(|error| {
        crate::error::web_scaffold_store_policy_unreadable(&database_path, &error)
    })?;
    database
        .release_policy("web")
        .map_err(|error| crate::error::web_scaffold_store_policy_unreadable(&database_path, &error))
}

#[cfg(feature = "web")]
fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
}

/// Select the newest stable registry release that survives the project's age gate.
/// Chọn bản stable mới nhất còn thỏa cổng tuổi phát hành của project.
#[cfg(feature = "web")]
pub(crate) fn select_latest_eligible_version(
    package: &str,
    metadata: &PackageMetadata,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let latest_tag = metadata
        .dist_tags
        .get("latest")
        .ok_or_else(|| crate::error::web_scaffold_latest_tag_missing(package))?;
    let latest = Version::parse(latest_tag)
        .with_context(|| crate::error::web_scaffold_latest_tag_invalid(package))?;
    let package_name = PackageName::new(package)?;
    let versions = eligible_versions(&package_name, metadata, age_policy.map(Into::into))
        .map_err(anyhow::Error::msg)?;
    versions
        .into_iter()
        .filter(|version| {
            version.pre.is_none()
                && version.dev.is_none()
                && version.post.is_none()
                && version <= &latest
        })
        .max()
        .map(|version| format!("^{version}"))
        .ok_or_else(|| crate::error::web_scaffold_no_eligible_release(package))
}

/// Keep a curated scaffold range only when it contains an eligible stable release.
/// Chỉ giữ range seed đã chọn khi nó còn chứa bản stable hợp lệ theo age gate.
#[cfg(feature = "web")]
pub(crate) fn validate_seed_range_age_eligible(
    package: &str,
    metadata: &PackageMetadata,
    age_policy: ScaffoldAgePolicy,
    requested_range: &str,
) -> Result<()> {
    let latest_tag = metadata
        .dist_tags
        .get("latest")
        .ok_or_else(|| crate::error::web_scaffold_latest_tag_missing(package))?;
    let latest = Version::parse(latest_tag)
        .with_context(|| crate::error::web_scaffold_latest_tag_invalid(package))?;
    let package_name = PackageName::new(package)?;
    let range = VersionRange::parse(requested_range)?;
    let versions = eligible_versions(&package_name, metadata, Some(age_policy.into()))
        .map_err(anyhow::Error::msg)?;
    if versions.into_iter().any(|version| {
        version.pre.is_none()
            && version.dev.is_none()
            && version.post.is_none()
            && version <= latest
            && range.matches(&version)
    }) {
        return Ok(());
    }
    Err(crate::error::web_scaffold_no_eligible_release_in_range(
        package,
        requested_range,
    ))
}

#[cfg(feature = "web")]
async fn fetch_age_gated_npm_metadata(package: &str) -> Result<PackageMetadata> {
    let registry = NpmRegistry::new(DEFAULT_NPM_REGISTRY);
    registry.set_age_gate_armed(true);
    registry
        .fetch_metadata(package)
        .await
        .map_err(|error| crate::error::web_scaffold_age_metadata_unavailable(package, &error))
}

/// Resolve or validate a scaffold seed version, preserving curated compatible ranges.
/// Resolve hoặc kiểm tra version seed, giữ nguyên range tương thích đã tuyển chọn.
#[cfg(feature = "web")]
pub(crate) async fn resolve_seed_dependency_version(
    package: &str,
    preferred_range: Option<&str>,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let metadata = fetch_age_gated_npm_metadata(package).await?;
        if let Some(range) = preferred_range {
            validate_seed_range_age_eligible(package, &metadata, policy, range)?;
            return Ok(range.to_string());
        }
        return select_latest_eligible_version(package, &metadata, Some(policy));
    }

    if let Some(range) = preferred_range {
        return Ok(range.to_string());
    }
    fetch_npm_latest_version(package, None).await
}

/// Resolve an explicitly selected npm dist-tag to its concrete package version.
/// Chuyển dist-tag do người dùng chọn thành version cụ thể của package.
#[cfg(feature = "web")]
pub(crate) async fn resolve_npm_dist_tag_version(
    package: &str,
    tag: &str,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let age_policy = age_policy.filter(|policy| policy.cutoff_hours > 0);
    let metadata = if age_policy.is_some() {
        fetch_age_gated_npm_metadata(package).await?
    } else {
        NpmRegistry::new(DEFAULT_NPM_REGISTRY)
            .fetch_metadata(package)
            .await
            .map_err(|error| crate::error::network_error_fetching(package, &error))?
    };
    selected_dist_tag_version(package, tag, &metadata, age_policy)
}

#[cfg(feature = "web")]
pub(super) fn selected_dist_tag_version(
    package: &str,
    tag: &str,
    metadata: &PackageMetadata,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let tagged_version = metadata
        .dist_tags
        .get(tag)
        .ok_or_else(|| crate::error::web_scaffold_dist_tag_missing(package, tag))?;
    let version = Version::parse(tagged_version)
        .map_err(|_| crate::error::web_scaffold_dist_tag_invalid(package, tag))?;

    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let package_name = PackageName::new(package)?;
        let eligible = eligible_versions(&package_name, metadata, Some(policy.into()))
            .map_err(anyhow::Error::msg)?;
        if !eligible.contains(&version) {
            return Err(crate::error::web_scaffold_dist_tag_not_eligible(
                package, tag, &version,
            ));
        }
    }

    Ok(version.to_string())
}

#[cfg(not(feature = "web"))]
pub(crate) async fn resolve_npm_dist_tag_version(
    package: &str,
    tag: &str,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    fetch_npm_selector_version_from_registry(DEFAULT_NPM_REGISTRY, package, tag).await
}

#[cfg(not(feature = "web"))]
pub(crate) async fn resolve_seed_dependency_version(
    package: &str,
    preferred_range: Option<&str>,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(range) = preferred_range {
        return Ok(range.to_string());
    }
    fetch_npm_latest_version(package, None).await
}

#[cfg(feature = "web")]
pub(crate) async fn fetch_npm_latest_version(
    package: &str,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let metadata = fetch_age_gated_npm_metadata(package).await?;
        return select_latest_eligible_version(package, &metadata, Some(policy));
    }

    fetch_un_gated_npm_latest_version(package).await
}

#[cfg(not(feature = "web"))]
pub(crate) async fn fetch_npm_latest_version(
    package: &str,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    fetch_un_gated_npm_latest_version(package).await
}

async fn fetch_un_gated_npm_latest_version(package: &str) -> Result<String> {
    if let Some(version) = scaffold_version_override(package) {
        return Ok(version);
    }

    match fetch_npm_latest_version_from_registry(DEFAULT_NPM_REGISTRY, package).await {
        Ok(version) => Ok(version),
        Err(error) => fallback_scaffold_version(package, error, None),
    }
}

pub(super) fn fallback_scaffold_version(
    package: &str,
    error: anyhow::Error,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if age_policy.is_some_and(|policy| policy.cutoff_hours > 0) {
        return Err(crate::error::web_scaffold_age_metadata_unavailable(
            package, &error,
        ));
    }
    if let Some(version) = scaffold_baseline_version(package) {
        return Ok(version.to_string());
    }
    eprintln!(
        "warning: could not resolve version for '{package}' ({}); using 'latest'",
        error
    );
    Ok("latest".to_string())
}

/// Shared HTTP client (pooled — one client for every version lookup).
/// HTTP client dùng chung (pool — một client cho mọi lần tra version).
fn global_cli_http_client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_max_idle_per_host(50)
            .pool_idle_timeout(std::time::Duration::from_secs(120))
            .tcp_keepalive(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(60))
            .user_agent(format!("MagiCore/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client")
    })
}

async fn fetch_npm_latest_version_from_registry(
    registry_url: &str,
    package: &str,
) -> Result<String> {
    let selected =
        fetch_npm_selector_version_from_registry(registry_url, package, "latest").await?;
    Ok(format!("^{selected}"))
}

async fn fetch_npm_selector_version_from_registry(
    registry_url: &str,
    package: &str,
    selector: &str,
) -> Result<String> {
    let url = format!(
        "{}/{package}/{selector}",
        registry_url.trim_end_matches('/')
    );
    let resp = global_cli_http_client()
        .get(&url)
        .send()
        .await
        .map_err(|e| crate::error::network_error_fetching(package, &e))?;
    if !resp.status().is_success() {
        return Err(crate::error::npm_registry_status(package, &resp.status()));
    }
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| crate::error::bad_npm_response(package, &e))?;
    body["version"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| crate::error::no_version_field(package))
}

fn scaffold_version_override(package: &str) -> Option<String> {
    std::env::var(SCAFFOLD_VERSION_OVERRIDES_ENV)
        .ok()
        .and_then(|raw| {
            raw.split(',')
                .filter_map(|entry| entry.trim().split_once('='))
                .find_map(|(name, version)| {
                    (name.trim() == package && !version.trim().is_empty())
                        .then(|| version.trim().to_string())
                })
        })
}

/// The embedded baseline table (parsed once, reused forever).
/// Bảng baseline nhúng (parse một lần, dùng mãi).
pub(crate) fn scaffold_baseline_versions() -> &'static HashMap<String, String> {
    static VERSIONS: std::sync::OnceLock<HashMap<String, String>> = std::sync::OnceLock::new();
    VERSIONS.get_or_init(|| {
        toml::from_str::<ScaffoldBaselineVersions>(SCAFFOLD_BASELINE_VERSIONS_TOML)
            .expect("templates/web/versions/scaffold-baseline.toml must be valid")
            .versions
    })
}

/// Baseline lookup for one package (None = not in the table).
/// Tra baseline một package (None = không có trong bảng).
pub(crate) fn scaffold_baseline_version(package: &str) -> Option<&'static str> {
    scaffold_baseline_versions()
        .get(package)
        .map(String::as_str)
}
