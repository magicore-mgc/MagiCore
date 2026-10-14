//! Crates.io sparse-index native engine (Rust).
//! Engine native sparse-index crates.io (Rust).
//!
//! Sparse index HTTP spec (RFC 2789): index entries are newline-delimited
//! JSON at `{index}/{prefix}/{name}` where the prefix rule is len 1→"1",
//! len 2→"2", len 3→"3/{first}", len≥4→"{first2}/{first3}". The artifact is
//! downloaded from `{download_base}/{name}/{version}/download` (a `.crate`
//! gzipped tarball) and its `sha256` verified against the index `cksum`.
//! Spec sparse-index (RFC 2789): entry index là JSON newline-delimited tại
//! `{index}/{prefix}/{name}` với luật prefix len 1→"1", len 2→"2",
//! len 3→"3/{first}", len≥4→"{first2}/{first3}". Artifact tải từ
//! `{download_base}/{name}/{version}/download` (tarball `.crate` gzip) và
//! `sha256` được xác minh theo `cksum` trong index.

use super::{GRAPH_RESOLVE_CONCURRENCY, RegistryProtocol, ResolvedEntry};
use async_trait::async_trait;
use futures_util::stream::{self, StreamExt, TryStreamExt};
use mgc_types::{MgError, MgResult, Version};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::PathBuf;
use std::time::Duration;

const DEFAULT_INDEX_URL: &str = "https://index.crates.io";
const DEFAULT_DOWNLOAD_BASE: &str = "https://crates.io/api/v1/crates";

/// Native crates.io sparse-index engine.
/// Engine native sparse-index crates.io.
#[derive(Debug, Clone)]
pub struct CratesProtocol {
    index_url: String,
    download_base: String,
    client: mgc_http::HttpClient,
}

impl CratesProtocol {
    /// Build with an explicit index URL (default download base).
    /// Dựng với index URL tường minh (download base mặc định).
    pub fn new(index_url: &str) -> Self {
        Self::with_download_base(index_url, DEFAULT_DOWNLOAD_BASE)
    }

    /// Build with explicit index + download base (testability).
    /// Dựng với index + download base tường minh (cho test).
    pub fn with_download_base(index_url: &str, download_base: &str) -> Self {
        let timeout = mgc_http::timeout::TimeoutConfig {
            // Some crates.io sparse-index documents exceed 1 MiB. Under
            // bounded parallel resolution they can take longer than the
            // generic 30s API timeout, so allow enough time for the body.
            request: Duration::from_secs(120),
            ..mgc_http::timeout::TimeoutConfig::default()
        };
        let client = mgc_http::HttpClient::with_security(&timeout, &mgc_http::TlsConfig::default())
            .expect("default crates.io HTTP/TLS policy is valid");
        Self {
            index_url: index_url.trim_end_matches('/').to_string(),
            download_base: download_base.trim_end_matches('/').to_string(),
            client,
        }
    }

    /// Build from environment: `MGC_CRATES_INDEX_URL` / `MGC_CRATES_DOWNLOAD_URL`.
    /// Dựng từ môi trường: `MGC_CRATES_INDEX_URL` / `MGC_CRATES_DOWNLOAD_URL`.
    pub fn from_env() -> Self {
        let index_url = std::env::var("MGC_CRATES_INDEX_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_INDEX_URL.to_string());
        let download_base = std::env::var("MGC_CRATES_DOWNLOAD_URL")
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| DEFAULT_DOWNLOAD_BASE.to_string());
        Self::with_download_base(&index_url, &download_base)
    }

    fn index_url_for(&self, name: &str) -> String {
        let name = name.to_ascii_lowercase();
        let prefix = sparse_prefix(&name);
        format!("{}/{prefix}/{name}", self.index_url)
    }

    fn download_url_for(&self, name: &str, version: &str) -> String {
        format!("{}/{name}/{version}/download", self.download_base)
    }

    async fn get_text(&self, url: &str) -> MgResult<String> {
        let (status, bytes) = self.get_response_body(url).await?;
        let body = String::from_utf8(bytes).map_err(|error| {
            MgError::Network(format!(
                "GET {url} returned a non-UTF-8 index body: {error}"
            ))
        })?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!(
                "GET {url} returned {status}: {body}"
            )));
        }
        Ok(body)
    }

    async fn get_bytes(&self, url: &str) -> MgResult<Vec<u8>> {
        let (status, bytes) = self.get_response_body(url).await?;
        if !(200..300).contains(&status) {
            return Err(MgError::Network(format!("GET {url} returned {status}")));
        }
        Ok(bytes)
    }

    /// Retry a response-body read after headers arrive. The shared HTTP
    /// client retries connection/status failures, but a truncated compressed
    /// body fails later in `Response::bytes()` and previously escaped without
    /// retrying. Index documents and crate archives are idempotent GETs.
    /// Retry phần đọc response body sau khi đã nhận headers. HTTP client dùng
    /// chung đã retry lỗi kết nối/status, nhưng body gzip bị cụt lỗi muộn ở
    /// `Response::bytes()` nên trước đây không được retry.
    async fn get_response_body(&self, url: &str) -> MgResult<(u16, Vec<u8>)> {
        const BODY_READ_RETRIES: u32 = 2;
        for attempt in 0..=BODY_READ_RETRIES {
            let response = self
                .client
                .get(url)
                .await
                .map_err(|error| MgError::Network(format!("GET {url} failed: {error}")))?;
            let status = response.status().as_u16();
            match response.bytes().await {
                Ok(bytes) => return Ok((status, bytes.to_vec())),
                Err(error) if attempt < BODY_READ_RETRIES => {
                    let delay = std::time::Duration::from_millis(200 * (1 << attempt));
                    tracing::warn!(
                        url,
                        attempt = attempt + 1,
                        "registry response body failed; retrying in {:?}: {}",
                        delay,
                        error
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(error) => {
                    return Err(MgError::Network(format!(
                        "read {url} response body failed after {} attempts: {error}",
                        BODY_READ_RETRIES + 1
                    )));
                }
            }
        }
        unreachable!("bounded response-body retry loop always returns")
    }

    /// Load every version row for one crate from the sparse index.
    /// Tải mọi hàng version của một crate từ sparse index.
    async fn index_entries(&self, name: &str) -> MgResult<Vec<CrateIndexEntry>> {
        let body = self.get_text(&self.index_url_for(name)).await?;
        body.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                serde_json::from_str(line).map_err(|error| {
                    MgError::Other(format!("parse crate index entry failed: {error}"))
                })
            })
            .collect()
    }

    /// Select one non-yanked version satisfying every currently known edge.
    /// Chọn version không yanked thỏa mọi ràng buộc hiện đã biết.
    fn select_index_entry(
        &self,
        name: &str,
        ranges: &[String],
        entries: Vec<CrateIndexEntry>,
    ) -> MgResult<CrateIndexEntry> {
        let mut best: Option<(Version, CrateIndexEntry)> = None;
        let mut best_yanked: Option<(Version, CrateIndexEntry)> = None;
        for entry in entries {
            if !entry.name.eq_ignore_ascii_case(name) {
                continue;
            }
            let Ok(version) = Version::parse(&entry.vers) else {
                continue;
            };
            if !ranges.iter().all(|range| cargo_matches(range, &version)) {
                continue;
            }
            let slot = if entry.yanked {
                &mut best_yanked
            } else {
                &mut best
            };
            if slot.as_ref().is_none_or(|(current, _)| version > *current) {
                *slot = Some((version, entry));
            }
        }
        let chosen = best.or_else(|| {
            if best_yanked.is_some() {
                eprintln!(
                    "WARNING: {name}@{} only matches yanked versions — refusing to resolve",
                    ranges.join(", ")
                );
            }
            None
        });
        let (_, entry) = chosen.ok_or_else(|| {
            MgError::Other(format!(
                "no non-yanked version of {name} matches constraints [{}]",
                ranges.join(", ")
            ))
        })?;
        let checksum = entry
            .cksum
            .as_deref()
            .map(str::trim)
            .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
            .or_else(|| {
                entry
                    .cksum
                    .as_deref()
                    .and_then(|value| value.strip_prefix("sha256:"))
                    .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
            })
            .map(str::to_ascii_lowercase)
            .ok_or_else(|| {
                MgError::Integrity(format!(
                    "crates.io index entry for {}@{} has no valid cksum; refusing to resolve an unverifiable artifact",
                    entry.name, entry.vers
                ))
        })?;
        let mut entry = entry;
        entry.cksum = Some(checksum);
        Ok(entry)
    }

    /// Resolve Cargo's non-dev lock closure for every target, including weakly
    /// referenced optional packages without activating their features.
    /// Resolve mọi target, bỏ dev; giữ package được weak-reference mà không bật feature.
    async fn resolve_feature_graph(
        &self,
        roots: &[super::RegistryRoot],
    ) -> MgResult<Vec<ResolvedEntry>> {
        let mut states = BTreeMap::<(String, String), CrateResolutionState>::new();
        let mut index_cache = BTreeMap::<String, Vec<CrateIndexEntry>>::new();
        let mut root_requests = BTreeMap::<String, CrateDependencyRequest>::new();
        for root in roots {
            let request = root_requests.entry(root.name.clone()).or_default();
            request.ranges.push(root.range.clone());
            request.features.extend(root.features.iter().cloned());
            request.default_features |= root.default_features;
        }

        for (name, request) in root_requests {
            let ranges = unique_strings(request.ranges);
            let entries = self.index_entries(&name).await?;
            let entry = self.select_index_entry(&name, &ranges, entries.clone())?;
            index_cache.insert(name, entries);
            let key = (entry.name.clone(), entry.vers.clone());
            states.insert(
                key,
                CrateResolutionState {
                    entry,
                    requested_features: request.features,
                    default_features: request.default_features,
                    active: true,
                    resolved_dependencies: BTreeSet::new(),
                    extra_markers: BTreeSet::new(),
                },
            );
        }

        loop {
            let mut pending = Vec::new();
            for (parent, state) in &states {
                let activation = activate_crate_features(
                    &state.entry,
                    &state.requested_features,
                    state.default_features,
                )?;
                for dependency in &state.entry.deps {
                    if dependency.kind.as_deref() == Some("dev") {
                        continue;
                    }
                    let alias = dependency.name.as_str();
                    let dependency_active = state.active
                        && (!dependency.optional
                            || activation.optional_dependencies.contains(alias));
                    let weak_lock_only = activation.lock_only_optional_dependencies.contains(alias);
                    let feature_selected = activation.optional_dependencies.contains(alias);
                    if dependency.optional
                        && !dependency_active
                        && !weak_lock_only
                        && !(feature_selected && !state.active)
                    {
                        continue;
                    }
                    if let Some(registry) = &dependency.registry {
                        return Err(MgError::Other(format!(
                            "crates.io package '{}'@{} requires dependency '{}' from alternate registry '{registry}', which MagiCore cannot resolve yet",
                            state.entry.name, state.entry.vers, dependency.name
                        )));
                    }
                    let actual_name = dependency
                        .package
                        .as_deref()
                        .unwrap_or(dependency.name.as_str())
                        .to_string();
                    let mut request = CrateDependencyRequest::default();
                    request.ranges.push(if dependency.req.trim().is_empty() {
                        "*".to_string()
                    } else {
                        dependency.req.clone()
                    });
                    request.features.extend(dependency.features.iter().cloned());
                    if let Some(features) = activation.dependency_features.get(alias) {
                        request.features.extend(features.iter().cloned());
                    }
                    request.default_features = dependency_active && dependency.default_features;
                    pending.push((parent.clone(), actual_name, request, dependency_active));
                }
            }

            let names_to_fetch = pending
                .iter()
                .map(|(_, name, _, _)| name.clone())
                .filter(|name| !index_cache.contains_key(name))
                .collect::<BTreeSet<_>>();
            let fetched = stream::iter(names_to_fetch.into_iter().map(|name| async move {
                let entries = self.index_entries(&name).await?;
                Ok::<_, MgError>((name, entries))
            }))
            .buffered(GRAPH_RESOLVE_CONCURRENCY)
            .try_collect::<Vec<_>>()
            .await?;
            index_cache.extend(fetched);

            let mut changed = false;
            let mut next_edges = BTreeMap::<(String, String), BTreeSet<(String, String)>>::new();
            for (parent, name, mut request, dependency_active) in pending {
                request.ranges = unique_strings(request.ranges);
                let entries = index_cache.get(&name).ok_or_else(|| {
                    MgError::Other("crates.io index cache lost a requested package".to_string())
                })?;
                let entry = self.select_index_entry(&name, &request.ranges, entries.clone())?;
                let key = (entry.name.clone(), entry.vers.clone());
                next_edges.entry(parent).or_default().insert(key.clone());
                if let Some(state) = states.get_mut(&key) {
                    if dependency_active && !state.active {
                        state.active = true;
                        changed = true;
                    }
                    for feature in request.features {
                        changed |= state.requested_features.insert(feature);
                    }
                    if request.default_features && !state.default_features {
                        state.default_features = true;
                        changed = true;
                    }
                } else {
                    states.insert(
                        key.clone(),
                        CrateResolutionState {
                            entry,
                            requested_features: request.features,
                            default_features: request.default_features,
                            active: dependency_active,
                            resolved_dependencies: BTreeSet::new(),
                            extra_markers: BTreeSet::new(),
                        },
                    );
                    changed = true;
                }
            }
            for (key, state) in &mut states {
                state.resolved_dependencies = next_edges.remove(key).unwrap_or_default();
                state.extra_markers.clear();
                if !state.active {
                    continue;
                }
                let activation = activate_crate_features(
                    &state.entry,
                    &state.requested_features,
                    state.default_features,
                )?;
                for dependency in &state.entry.deps {
                    if dependency.kind.as_deref() == Some("dev") {
                        continue;
                    }
                    if let Some(target) = &dependency.target {
                        state.extra_markers.insert(format!("target:{target}"));
                    }
                    if dependency.optional
                        && !activation
                            .optional_dependencies
                            .contains(dependency.name.as_str())
                    {
                        let actual_name = dependency
                            .package
                            .as_deref()
                            .unwrap_or(dependency.name.as_str());
                        state
                            .extra_markers
                            .insert(format!("optional:{actual_name}"));
                    }
                }
            }
            if !changed {
                break;
            }
        }

        states
            .values()
            .map(|state| resolved_entry_for_state(self, state))
            .collect()
    }

    /// Materialize a crate into the cargo layout: `.crate` in
    /// `{cargo_home}/registry/cache/` and extracted sources in
    /// `{cargo_home}/registry/src/{name}-{version}/`. Returns the `.crate` path.
    /// Materialize crate vào layout cargo: `.crate` trong
    /// `{cargo_home}/registry/cache/` và source đã giải nén trong
    /// `{cargo_home}/registry/src/{name}-{version}/`. Trả path `.crate`.
    pub fn materialize(
        &self,
        entry: &ResolvedEntry,
        bytes: &[u8],
        cargo_home: &std::path::Path,
    ) -> MgResult<PathBuf> {
        let cache_dir = cargo_home.join("registry").join("cache");
        std::fs::create_dir_all(&cache_dir)?;
        let crate_path = cache_dir.join(format!("{}-{}.crate", entry.name, entry.version));
        std::fs::write(&crate_path, bytes)?;

        let src_root = cargo_home.join("registry").join("src");
        super::archive::extract_crate_tar_gz(
            bytes,
            &src_root,
            &format!("{}-{}", entry.name, entry.version),
        )?;
        Ok(crate_path)
    }
}

impl Default for CratesProtocol {
    fn default() -> Self {
        Self::from_env()
    }
}

#[async_trait]
impl RegistryProtocol for CratesProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        cargo_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        let index_url = self.index_url_for(name);
        let body = self.get_text(&index_url).await?;

        let mut best: Option<(Version, CrateIndexEntry)> = None;
        let mut best_yanked: Option<(Version, CrateIndexEntry)> = None;

        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let entry: CrateIndexEntry = serde_json::from_str(line)
                .map_err(|e| MgError::Other(format!("parse crate index entry failed: {e}")))?;
            if !entry.name.eq_ignore_ascii_case(name) {
                continue;
            }
            let Ok(version) = Version::parse(&entry.vers) else {
                continue;
            };
            if !cargo_matches(range, &version) {
                continue;
            }
            if entry.yanked {
                if best_yanked.as_ref().is_none_or(|(v, _)| version > *v) {
                    best_yanked = Some((version, entry));
                }
                continue;
            }
            if best.as_ref().is_none_or(|(v, _)| version > *v) {
                best = Some((version, entry));
            }
        }

        let chosen = best.or_else(|| {
            // Never select a yanked version; if only yanked candidates
            // match, warn and fail closed (the dependency is skipped by the
            // caller, never silently pinned to a yanked artifact).
            // Không bao giờ chọn version yanked; nếu chỉ còn ứng viên yanked
            // khớp, cảnh báo và fail-closed (caller bỏ dependency, không bao
            // giờ âm thầm ghim vào artifact yanked).
            if best_yanked.is_some() {
                eprintln!(
                    "WARNING: {name}@{range} only matches yanked versions — skipping (fail-closed)"
                );
            }
            None
        });

        let entry = chosen.map(|(_, e)| e).ok_or_else(|| {
            MgError::Other(format!(
                "no non-yanked version of {name} matches range '{range}'"
            ))
        })?;

        let artifact_url = self.download_url_for(name, &entry.vers);
        // The REAL sparse index carries a BARE 64-hex `cksum` (no
        // scheme prefix) — an earlier revision only accepted a
        // `sha256:`-prefixed value and dropped every genuine checksum.
        // (Index thật mang `cksum` hex trần, không prefix.)
        let sha256 = entry
            .cksum
            .as_deref()
            .map(str::trim)
            .filter(|c| c.len() == 64 && c.chars().all(|ch| ch.is_ascii_hexdigit()))
            .or_else(|| {
                entry
                    .cksum
                    .as_deref()
                    .and_then(|c| c.strip_prefix("sha256:"))
                    .filter(|c| c.len() == 64 && c.chars().all(|ch| ch.is_ascii_hexdigit()))
            })
            .map(|c| c.to_ascii_lowercase())
            .ok_or_else(|| {
                MgError::Integrity(format!(
                    "crates.io index entry for {}@{} has no valid cksum; refusing to resolve an unverifiable artifact",
                    entry.name, entry.vers
                ))
            })?;

        let mut resolved = ResolvedEntry {
            name: name.to_string(),
            version: entry.vers.clone(),
            deps: Vec::new(),
            artifact_url,
            sha256,
            extra_markers: Vec::new(),
        };

        for dep in &entry.deps {
            let kind = dep.kind.as_deref().unwrap_or("normal");
            // dev deps are excluded from the graph — only normal + build.
            // (dev deps bị loại khỏi graph — chỉ normal + build.)
            if kind == "dev" {
                continue;
            }
            let actual_name = dep.package.clone().unwrap_or_else(|| dep.name.clone());
            // Optional deps are not in the selected graph unless a feature
            // activates them. Keep an audit marker, but do not resolve or
            // download dormant packages.
            // (Optional dep chưa thuộc graph được chọn nếu feature chưa bật;
            // giữ marker audit nhưng không resolve/tải package đang ngủ.)
            if dep.optional {
                resolved
                    .extra_markers
                    .push(format!("optional:{actual_name}"));
                continue;
            }
            if let Some(registry) = &dep.registry {
                return Err(MgError::Other(format!(
                    "crates.io package '{}'@{} requires dependency '{}' from alternate registry '{registry}', which MagiCore cannot resolve yet",
                    name, entry.vers, dep.name
                )));
            }
            // Cargo.lock is shared across target platforms. Include every
            // target-qualified edge so builds on another OS can stay locked;
            // the source Cargo.toml still decides whether Cargo compiles it.
            // (Cargo.lock dùng chung giữa các target; giữ mọi cạnh có điều
            // kiện để build OS khác vẫn locked; Cargo.toml quyết định compile.)
            if let Some(target) = &dep.target {
                resolved.extra_markers.push(format!("target:{target}"));
            }
            let req = if dep.req.trim().is_empty() {
                "*".to_string()
            } else {
                dep.req.clone()
            };
            resolved.deps.push((actual_name, req));
        }

        Ok(resolved)
    }

    async fn resolve_graph(&self, name: &str, range: &str) -> MgResult<Vec<ResolvedEntry>> {
        self.resolve_feature_graph(&[super::RegistryRoot {
            name: name.to_string(),
            range: range.to_string(),
            features: Vec::new(),
            default_features: true,
        }])
        .await
    }

    async fn resolve_graph_roots(
        &self,
        roots: &[(String, String)],
    ) -> MgResult<Vec<ResolvedEntry>> {
        let roots = roots
            .iter()
            .map(|(name, range)| super::RegistryRoot {
                name: name.clone(),
                range: range.clone(),
                features: Vec::new(),
                default_features: true,
            })
            .collect::<Vec<_>>();
        self.resolve_feature_graph(&roots).await
    }

    async fn resolve_graph_roots_with_options(
        &self,
        roots: &[super::RegistryRoot],
    ) -> MgResult<Vec<ResolvedEntry>> {
        self.resolve_feature_graph(roots).await
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        self.get_bytes(&entry.artifact_url).await
    }
}

/// Sparse-index prefix rule for a crate name (the path component BEFORE the
/// final `/{name}` segment, which `index_url_for` appends).
/// Luật prefix sparse-index cho tên crate (thành phần path TRƯỚC đoạn
/// `/{name}` cuối, do `index_url_for` tự nối thêm).
fn sparse_prefix(name: &str) -> String {
    match name.len() {
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        3 => format!("3/{}", &name[..1]),
        _ => format!("{}/{}", &name[..2], &name[2..4]),
    }
}

/// Use Cargo-compatible SemVer parsing rather than a partial hand-written matcher.
/// Dùng parser SemVer cùng quy tắc Cargo thay cho matcher tự viết chưa đầy đủ.
fn cargo_matches(range: &str, version: &Version) -> bool {
    let Ok(candidate) = semver::Version::parse(&version.to_string()) else {
        return false;
    };
    semver::VersionReq::parse(range.trim()).is_ok_and(|requirement| requirement.matches(&candidate))
}

/// One crate version entry from the sparse index NDJSON.
/// Một entry version crate từ NDJSON sparse-index.
#[derive(Debug, Clone, Deserialize)]
struct CrateIndexEntry {
    name: String,
    vers: String,
    #[serde(default)]
    deps: Vec<CrateDep>,
    #[serde(default)]
    features: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    features2: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    cksum: Option<String>,
    #[serde(default)]
    yanked: bool,
}

/// One dependency edge in a crate version entry.
/// Một cạnh dependency trong entry version crate.
#[derive(Debug, Clone, Deserialize)]
struct CrateDep {
    name: String,
    #[serde(default)]
    req: String,
    #[serde(default)]
    features: Vec<String>,
    #[serde(default = "default_true")]
    default_features: bool,
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    optional: bool,
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    package: Option<String>,
    #[serde(default)]
    registry: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Default)]
struct CrateDependencyRequest {
    ranges: Vec<String>,
    features: BTreeSet<String>,
    default_features: bool,
}

struct CrateResolutionState {
    entry: CrateIndexEntry,
    requested_features: BTreeSet<String>,
    default_features: bool,
    /// Whether requested features activate this package, or it is retained
    /// only because an active feature weakly references its optional edge.
    active: bool,
    resolved_dependencies: BTreeSet<(String, String)>,
    extra_markers: BTreeSet<String>,
}

#[derive(Default)]
struct ActivatedCrateFeatures {
    optional_dependencies: BTreeSet<String>,
    lock_only_optional_dependencies: BTreeSet<String>,
    dependency_features: BTreeMap<String, BTreeSet<String>>,
}

/// Expand Cargo index feature syntax into optional edges and child features.
/// Mở rộng cú pháp feature trong index Cargo thành cạnh optional và feature con.
fn activate_crate_features(
    entry: &CrateIndexEntry,
    requested_features: &BTreeSet<String>,
    default_features: bool,
) -> MgResult<ActivatedCrateFeatures> {
    let mut features = entry.features.clone();
    for (name, values) in &entry.features2 {
        features
            .entry(name.clone())
            .or_default()
            .extend(values.iter().cloned());
    }
    let optional_aliases = entry
        .deps
        .iter()
        .filter(|dependency| dependency.optional && dependency.kind.as_deref() != Some("dev"))
        .map(|dependency| dependency.name.clone())
        .collect::<BTreeSet<_>>();
    let explicitly_named_optional = features
        .values()
        .flatten()
        .filter_map(|value| value.strip_prefix("dep:"))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let implicit_optional = optional_aliases
        .difference(&explicitly_named_optional)
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut activation = ActivatedCrateFeatures::default();
    let mut pending = requested_features.iter().cloned().collect::<VecDeque<_>>();
    if default_features {
        pending.push_back("default".to_string());
    }
    let mut expanded = BTreeSet::new();
    let mut weak_features = BTreeMap::<String, BTreeSet<String>>::new();

    while let Some(feature) = pending.pop_front() {
        if !expanded.insert(feature.clone()) {
            continue;
        }
        let Some(values) = features.get(&feature) else {
            if implicit_optional.contains(&feature) {
                activation.optional_dependencies.insert(feature);
                continue;
            }
            if feature == "default" && default_features {
                continue;
            }
            return Err(MgError::Other(format!(
                "crates.io package '{}'@{} does not define requested feature '{feature}'",
                entry.name, entry.vers
            )));
        };
        for value in values {
            if let Some(alias) = value.strip_prefix("dep:") {
                if !entry.deps.iter().any(|dependency| dependency.name == alias) {
                    return Err(MgError::Other(format!(
                        "crates.io feature '{feature}' references missing dependency '{alias}' in '{}'@{}",
                        entry.name, entry.vers
                    )));
                }
                activation.optional_dependencies.insert(alias.to_string());
                continue;
            }
            if let Some((dependency, child_feature)) = value.split_once('/') {
                let weak = dependency.ends_with('?');
                let alias = dependency.strip_suffix('?').unwrap_or(dependency);
                if alias.is_empty()
                    || child_feature.is_empty()
                    || !entry.deps.iter().any(|edge| edge.name == alias)
                {
                    return Err(MgError::Other(format!(
                        "crates.io feature '{feature}' has invalid dependency feature '{value}' in '{}'@{}",
                        entry.name, entry.vers
                    )));
                }
                if weak {
                    weak_features
                        .entry(alias.to_string())
                        .or_default()
                        .insert(child_feature.to_string());
                } else {
                    activation.optional_dependencies.insert(alias.to_string());
                    activation
                        .dependency_features
                        .entry(alias.to_string())
                        .or_default()
                        .insert(child_feature.to_string());
                }
                continue;
            }
            if features.contains_key(value) {
                pending.push_back(value.clone());
            } else if implicit_optional.contains(value) {
                activation.optional_dependencies.insert(value.clone());
            } else {
                return Err(MgError::Other(format!(
                    "crates.io feature '{feature}' references unknown feature or dependency '{value}' in '{}'@{}",
                    entry.name, entry.vers
                )));
            }
        }
    }

    for (alias, child_features) in weak_features {
        let dependency_is_active = entry
            .deps
            .iter()
            .any(|dependency| dependency.name == alias && !dependency.optional)
            || activation.optional_dependencies.contains(&alias);
        if dependency_is_active {
            activation
                .dependency_features
                .entry(alias)
                .or_default()
                .extend(child_features);
        } else if entry
            .deps
            .iter()
            .any(|dependency| dependency.name == alias && dependency.optional)
        {
            activation.lock_only_optional_dependencies.insert(alias);
        }
    }
    Ok(activation)
}

/// Convert a settled feature state into lock edges without losing dormant markers.
/// Chuyển feature state ổn định thành cạnh lock, vẫn giữ marker optional dormant.
fn resolved_entry_for_state(
    protocol: &CratesProtocol,
    state: &CrateResolutionState,
) -> MgResult<ResolvedEntry> {
    let sha256 = state
        .entry
        .cksum
        .as_deref()
        .ok_or_else(|| MgError::Integrity("crates.io index entry has no checksum".to_string()))?
        .to_ascii_lowercase();
    Ok(ResolvedEntry {
        name: state.entry.name.clone(),
        version: state.entry.vers.clone(),
        deps: state
            .resolved_dependencies
            .iter()
            .map(|(name, version)| (name.clone(), format!("={version}")))
            .collect(),
        artifact_url: protocol.download_url_for(&state.entry.name, &state.entry.vers),
        sha256,
        extra_markers: state.extra_markers.iter().cloned().collect(),
    })
}

fn unique_strings(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use mgc_types::Version;

    #[test]
    fn sparse_prefix_rules() {
        assert_eq!(sparse_prefix("a"), "1");
        assert_eq!(sparse_prefix("ab"), "2");
        assert_eq!(sparse_prefix("abc"), "3/a");
        assert_eq!(sparse_prefix("serde"), "se/rd");
    }

    #[test]
    fn cargo_caret_semantics() {
        let v = Version::parse("1.2.5").unwrap();
        assert!(cargo_matches("^1.2.3", &v));
        assert!(!cargo_matches("^1.2.3", &Version::parse("2.0.0").unwrap()));
        assert!(cargo_matches("1.2", &v)); // bare = caret
        assert!(cargo_matches(">=1.0, <2.0", &v));
        assert!(cargo_matches("~1.2.3", &v));
        assert!(!cargo_matches("~1.2.3", &Version::parse("1.3.0").unwrap()));
        assert!(cargo_matches("=1.2.5", &v));
        assert!(cargo_matches(
            "^0.25.0",
            &Version::parse("0.25.17+spec-1.1.0").unwrap()
        ));
    }
}
