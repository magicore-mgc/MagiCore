//! Enforce `mgc.toml [[sources]]` claims on the packages an install resolves.
//! Thực thi claim `mgc.toml [[sources]]` trên package mà install đã resolve.

use std::path::Path;

use mgc_config::project::SourceConfig;
use mgc_types::adapter::ResolvedPackage;

use crate::supply_chain::error::SupplyChainError;

/// The registry this install really talks to.
/// Registry mà lần install này thật sự gọi tới.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RegistryOrigin<'a> {
    /// Configured registry base URL (the chain head).
    /// Base URL registry đã cấu hình (đầu chain).
    pub(crate) url: &'a str,
    /// Ecosystem tag the source must serve (`"web"`).
    /// Tag ecosystem mà source phải phục vụ.
    pub(crate) ecosystem: &'a str,
}

/// A source the lockfile records, derived from project config.
/// Source được lockfile ghi, suy từ cấu hình project.
pub(crate) fn project_lockfile_sources(
    project_root: &Path,
    registry: RegistryOrigin<'_>,
) -> Result<Vec<mgc_lockfile::v4::SourceRef>, SupplyChainError> {
    let manifest = load_sources(project_root)?;
    let Some(sources) = manifest else {
        // No project sources: record the registry that really served the lane.
        // (Không có `[[sources]]`: ghi registry thật đã phục vụ lane.)
        return Ok(vec![mgc_lockfile::v4_graph::lane_source(
            registry.url,
            ecosystem_tag(registry.ecosystem),
        )]);
    };
    let registry_id = mgc_lockfile::v4_graph::source_id_for(registry.url.trim());
    Ok(sources
        .into_iter()
        .map(|source| {
            let trusted = source.trusted;
            mgc_lockfile::v4::SourceRef {
                // A source pointing at the configured registry keeps the
                // builder-derived id, so package rows and the source table
                // agree; every other configured source gets its declared id.
                // (Source trỏ đúng registry cấu hình giữ id do builder suy ra
                // để row package và bảng source khớp nhau.)
                id: if same_registry(&source.url, registry.url) {
                    registry_id.clone()
                } else {
                    source.id
                },
                url: source.url,
                ecosystem: if source.ecosystem.is_empty() {
                    ecosystem_tag(registry.ecosystem)
                } else {
                    ecosystem_tag(&source.ecosystem)
                },
                priority: source.priority,
                claims: source.claims,
                trusted,
                allow_hosts: source.allow_hosts,
                allow_cidrs: source.allow_cidrs,
                allow_protocols: source.allow_protocols,
            }
        })
        .collect())
}

/// Refuse a resolved package that a project source claims but this install
/// cannot attribute. Fail-closed: a project that names an internal registry
/// for `@corp/*` must never silently accept that scope from the public chain.
/// (Từ chối package đã resolve mà source của project tuyên bố nhưng lần install
/// này không quy được về source đó — fail-closed.)
pub(crate) fn ensure_resolved_sources_satisfy_claims(
    project_root: &Path,
    packages: &[ResolvedPackage],
    registry: RegistryOrigin<'_>,
) -> Result<(), SupplyChainError> {
    // No `[[sources]]` = the project made no claim, so nothing to enforce.
    // (Không có `[[sources]]` = project không tuyên bố gì, không có gì phải thực thi.)
    let Some(sources) = load_sources(project_root)? else {
        return Ok(());
    };
    let configured: Vec<&SourceConfig> = sources
        .iter()
        .filter(|source| {
            source.ecosystem.is_empty() || source.ecosystem.eq_ignore_ascii_case(registry.ecosystem)
        })
        .collect();

    for package in packages {
        let name = package.id.name_str();
        let Some(claimed) = configured
            .iter()
            .find(|source| source.claims.iter().any(|claim| claim_matches(claim, name)))
        else {
            continue;
        };
        if serves(claimed, package, registry) {
            continue;
        }
        return Err(SupplyChainError::SourceClaimNotServed {
            name: name.to_owned(),
            version: package.id.version().to_string(),
            source_id: claimed.id.clone(),
            source_url: claimed.url.clone(),
            registry: registry.url.to_owned(),
        });
    }
    Ok(())
}

/// Whether the claiming source actually served this package.
/// Source tuyên bố có thật sự phục vụ package này không.
fn serves(source: &SourceConfig, package: &ResolvedPackage, registry: RegistryOrigin<'_>) -> bool {
    let Ok(source_url) = url::Url::parse(&source.url) else {
        // An unusable source URL cannot serve anything: fail closed.
        // URL source dùng không được thì không phục vụ được gì: fail-closed.
        return false;
    };
    // The artifact must come from the claimed source's own origin when the
    // registry reports it; anything else would be a cross-source swap.
    // (Artifact phải tới từ đúng origin của source khi registry báo; khác là
    // đổi nguồn chéo.)
    match origin_host(&package.tarball_url) {
        Some(artifact_host) => {
            let claimed_host = source_url.host_str().unwrap_or_default();
            !claimed_host.is_empty() && artifact_host.eq_ignore_ascii_case(claimed_host)
        }
        // No artifact URL to attribute: the configured registry must be the
        // claimed source's own origin.
        // (Không có URL artifact để quy nguồn: registry cấu hình phải cùng
        // origin với source tuyên bố.)
        None => same_origin(&source_url, registry.url),
    }
}

fn origin_host(url: &str) -> Option<String> {
    url::Url::parse(url).ok()?.host_str().map(str::to_owned)
}

fn same_registry(declared: &str, configured: &str) -> bool {
    normalize(declared) == normalize(configured)
}

fn same_origin(source: &url::Url, configured: &str) -> bool {
    let Ok(configured) = url::Url::parse(configured) else {
        return false;
    };
    source.scheme() == configured.scheme()
        && source
            .host_str()
            .zip(configured.host_str())
            .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
}

fn normalize(url: &str) -> &str {
    url.trim().trim_end_matches('/')
}

/// Match a claim pattern against a package name, npm-style.
/// Khớp pattern claim với tên package theo kiểu npm.
fn claim_matches(pattern: &str, name: &str) -> bool {
    mgc_lockfile::claim_matches(pattern, name)
}

/// Map a project ecosystem string onto the lock schema tag (Web is the
/// only lane that reaches here; anything else stays unattributed).
/// (Ánh xạ ecosystem project sang tag schema lock; chỉ lane web tới đây.)
fn ecosystem_tag(name: &str) -> mgc_lockfile::EcosystemTag {
    if name.eq_ignore_ascii_case("web") {
        mgc_lockfile::EcosystemTag::Web
    } else {
        mgc_lockfile::EcosystemTag::Other
    }
}

fn load_sources(project_root: &Path) -> Result<Option<Vec<SourceConfig>>, SupplyChainError> {
    let config = mgc_config::project::ProjectConfig::load(project_root).map_err(|source| {
        SupplyChainError::ProjectSecurityConfig {
            path: project_root.join("mgc.toml"),
            source,
        }
    })?;
    Ok(config.and_then(|config| config.sources))
}
