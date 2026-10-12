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
        Self {
            index_url: index_url.trim_end_matches('/').to_string(),
            download_base: download_base.trim_end_matches('/').to_string(),
            client: mgc_http::HttpClient::default(),
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
        let resp = self
            .client
            .get(url)
            .await
            .map_err(|e| MgError::Network(format!("GET {url} failed: {e}")))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| MgError::Network(format!("read {url} body failed: {e}")))?;
        if !status.is_success() {
            return Err(MgError::Network(format!(
                "GET {url} returned {status}: {body}"
            )));
        }
        Ok(body)
    }

    async fn get_bytes(&self, url: &str) -> MgResult<Vec<u8>> {
        let resp = self
            .client
            .get(url)
            .await
            .map_err(|e| MgError::Network(format!("GET {url} failed: {e}")))?;
        let status = resp.status();
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| MgError::Network(format!("read {url} body failed: {e}")))?;
        if !status.is_success() {
            return Err(MgError::Network(format!("GET {url} returned {status}")));
        }
        Ok(bytes.to_vec())
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

    /// Resolve the Cargo feature closure without activating dormant optional dependencies.
    /// Resolve feature closure Cargo mà không bật optional dependency còn dormant.
    async fn resolve_feature_graph(
        &self,
        roots: &[(String, String)],
    ) -> MgResult<Vec<ResolvedEntry>> {
        let mut states = BTreeMap::<String, CrateResolutionState>::new();
        let mut root_requests = BTreeMap::<String, CrateDependencyRequest>::new();
        for (name, range) in roots {
            let request = root_requests.entry(name.clone()).or_default();
            request.ranges.push(range.clone());
            request.default_features = true;
        }

        for (name, request) in root_requests {
            let ranges = unique_strings(request.ranges);
            let entry =
                self.select_index_entry(&name, &ranges, self.index_entries(&name).await?)?;
            states.insert(
                name,
                CrateResolutionState {
                    entry,
                    ranges,
                    requested_features: request.features,
                    default_features: request.default_features,
                },
            );
        }

        loop {
            let mut pending = BTreeMap::<String, CrateDependencyRequest>::new();
            for state in states.values() {
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
                    if dependency.optional && !activation.optional_dependencies.contains(alias) {
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
                    let request = pending.entry(actual_name).or_default();
                    request.ranges.push(dependency.req.clone());
                    request.features.extend(dependency.features.iter().cloned());
                    if let Some(features) = activation.dependency_features.get(alias) {
                        request.features.extend(features.iter().cloned());
                    }
                    request.default_features |= dependency.default_features;
                }
            }

            let (known, missing): (Vec<_>, Vec<_>) = pending
                .into_iter()
                .partition(|(name, _)| states.contains_key(name));
            let mut changed = false;
            for (name, mut request) in known {
                request.ranges = unique_strings(request.ranges);
                let state = states.get_mut(&name).ok_or_else(|| {
                    MgError::Other("crates.io feature resolver lost a selected package".to_string())
                })?;
                for range in &request.ranges {
                    let version = Version::parse(&state.entry.vers).map_err(|error| {
                        MgError::Other(format!(
                            "invalid crates.io version '{}': {error}",
                            state.entry.vers
                        ))
                    })?;
                    if !cargo_matches(range, &version) {
                        return Err(MgError::DependencyConflict(format!(
                            "incompatible constraints for {name}: selected {}, but constraint '{range}' does not match (native crates resolver does not backtrack yet)",
                            state.entry.vers
                        )));
                    }
                }
                for range in request.ranges {
                    if !state.ranges.contains(&range) {
                        state.ranges.push(range);
                        changed = true;
                    }
                }
                for feature in request.features {
                    changed |= state.requested_features.insert(feature);
                }
                if request.default_features && !state.default_features {
                    state.default_features = true;
                    changed = true;
                }
            }
            let resolved_missing =
                stream::iter(missing.into_iter().map(|(name, request)| async move {
                    let ranges = unique_strings(request.ranges);
                    let entry =
                        self.select_index_entry(&name, &ranges, self.index_entries(&name).await?)?;
                    Ok::<_, MgError>((
                        name,
                        CrateResolutionState {
                            entry,
                            ranges,
                            requested_features: request.features,
                            default_features: request.default_features,
                        },
                    ))
                }))
                .buffered(GRAPH_RESOLVE_CONCURRENCY)
                .try_collect::<Vec<_>>()
                .await?;
            changed |= !resolved_missing.is_empty();
            states.extend(resolved_missing);
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
        self.resolve_feature_graph(&[(name.to_string(), range.to_string())])
            .await
    }

    async fn resolve_graph_roots(
        &self,
        roots: &[(String, String)],
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

/// Cargo semver subset matcher: `*`, `^x.y.z`, `~x.y.z`, `=x.y.z`,
/// `>=x`, `<=x`, `>x`, `<x`, comma = AND, bare version = caret.
/// Matcher semver subset Cargo: `*`, `^`, `~`, `=`, `>=`, `<=`, `>`, `<`,
/// phẩy = AND, version trần = caret.
fn cargo_matches(range: &str, version: &Version) -> bool {
    let range = range.trim();
    if range.is_empty() || range == "*" {
        return true;
    }
    range
        .split(',')
        .map(str::trim)
        .all(|part| cargo_part(part, version))
}

fn cargo_part(part: &str, version: &Version) -> bool {
    if part == "*" {
        return true;
    }
    if let Some(p) = part.strip_prefix('^') {
        return caret(p, version);
    }
    if let Some(p) = part.strip_prefix('~') {
        return tilde(p, version);
    }
    if let Some(p) = part.strip_prefix('=') {
        return exact(p, version);
    }
    if let Some(p) = part.strip_prefix(">=") {
        return ge(p, version);
    }
    if let Some(p) = part.strip_prefix("<=") {
        return le(p, version);
    }
    if let Some(p) = part.strip_prefix('>') {
        return gt(p, version);
    }
    if let Some(p) = part.strip_prefix('<') {
        return lt(p, version);
    }
    // bare version → caret semantics (cargo default).
    // (version trần → ngữ nghĩa caret (mặc định cargo).)
    caret(part, version)
}

fn caret(raw: &str, version: &Version) -> bool {
    let Some(target) = Version::parse(raw.trim()).ok() else {
        return false;
    };
    if version < &target {
        return false;
    }
    // ^1.2.3 → <2.0.0; ^0.2.3 → <0.3.0; ^0.0.3 → <0.0.4.
    if target.major > 0 {
        version.major == target.major
    } else if target.minor > 0 {
        version.major == 0 && version.minor == target.minor
    } else {
        version.major == 0 && version.minor == 0 && version.patch == target.patch
    }
}

fn tilde(raw: &str, version: &Version) -> bool {
    let Some(target) = Version::parse(raw.trim()).ok() else {
        return false;
    };
    version >= &target && version.major == target.major && version.minor == target.minor
}

fn exact(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|target| version == &target)
        .unwrap_or(false)
}

fn ge(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|target| version >= &target)
        .unwrap_or(false)
}

fn le(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|target| version <= &target)
        .unwrap_or(false)
}

fn gt(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|target| version > &target)
        .unwrap_or(false)
}

fn lt(raw: &str, version: &Version) -> bool {
    Version::parse(raw.trim())
        .map(|target| version < &target)
        .unwrap_or(false)
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
    ranges: Vec<String>,
    requested_features: BTreeSet<String>,
    default_features: bool,
}

#[derive(Default)]
struct ActivatedCrateFeatures {
    optional_dependencies: BTreeSet<String>,
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
    let activation = activate_crate_features(
        &state.entry,
        &state.requested_features,
        state.default_features,
    )?;
    let mut dependencies = BTreeMap::<String, String>::new();
    let mut markers = BTreeSet::new();
    for dependency in &state.entry.deps {
        if dependency.kind.as_deref() == Some("dev") {
            continue;
        }
        if let Some(target) = &dependency.target {
            markers.insert(format!("target:{target}"));
        }
        let actual_name = dependency
            .package
            .as_deref()
            .unwrap_or(dependency.name.as_str());
        if dependency.optional
            && !activation
                .optional_dependencies
                .contains(dependency.name.as_str())
        {
            markers.insert(format!("optional:{actual_name}"));
            continue;
        }
        dependencies
            .entry(actual_name.to_string())
            .or_insert_with(|| dependency.req.clone());
    }
    let sha256 = state
        .entry
        .cksum
        .as_deref()
        .ok_or_else(|| MgError::Integrity("crates.io index entry has no checksum".to_string()))?
        .to_ascii_lowercase();
    Ok(ResolvedEntry {
        name: state.entry.name.clone(),
        version: state.entry.vers.clone(),
        deps: dependencies.into_iter().collect(),
        artifact_url: protocol.download_url_for(&state.entry.name, &state.entry.vers),
        sha256,
        extra_markers: markers.into_iter().collect(),
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
    }
}
