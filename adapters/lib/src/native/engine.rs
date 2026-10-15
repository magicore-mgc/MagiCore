//! Native registry engine wiring for the lib adapter (rust/python).
//! Đi dây engine registry native cho lib adapter (rust/python).
//!
//! Turns a `RegistryProtocol` engine's transitive resolution into BOTH the
//! `ResolvedGraph` (for the `DependencyResolver::resolve` trait) AND the
//! mgc.lock v3 `Package` entries (ecosystem/provenance/registry/markers/
//! artifact). The blake3 `content_hash` + `store_ref` are filled at install
//! time (only the download can produce them); resolve records everything the
//! registry index itself vouches for.
//! Chuyển kết quả resolve bắc cầu của engine `RegistryProtocol` thành CẢ
//! `ResolvedGraph` (cho trait `DependencyResolver::resolve`) LẪN entry
//! `Package` v3 của mgc.lock (ecosystem/provenance/registry/markers/artifact).
//! `content_hash` blake3 + `store_ref` điền lúc install (chỉ download mới sinh
//! ra được); resolve ghi mọi thứ mà chính registry index đảm bảo.

use base64::Engine as _;
use mgc_lockfile::{ArtifactRef, EcosystemTag, Package, Provenance, SOURCE_KIND_NATIVE_RESOLVE};
use mgc_resolver::protocols::{RegistryProtocol, RegistryRoot, ResolvedEntry};
use mgc_types::{
    Manifest, MgResult, PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version,
};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

/// A native resolution: the graph for install + the v3 lock entries.
/// Một lần resolve native: graph cho install + entry lock v3.
#[derive(Debug, Clone)]
pub struct NativeResolution {
    pub graph: ResolvedGraph,
    pub lock_packages: Vec<Package>,
}

/// Resolve a manifest's regular dependencies through a native protocol engine.
/// Resolve dependencies thường của manifest qua một engine protocol native.
pub async fn resolve_with_protocol(
    protocol: &dyn RegistryProtocol,
    ecosystem: EcosystemTag,
    registry: &str,
    manifest: &Manifest,
) -> MgResult<NativeResolution> {
    resolve_with_protocol_inner(protocol, ecosystem, registry, manifest, false).await
}

/// Resolve both runtime and development dependencies through a native protocol.
/// Resolve cả dependency runtime và development qua protocol native.
///
/// Native Python and App/Flutter callers need dev packages in the installed
/// graph so test/build tools stay in the MGC lock and do not resolve separately.
/// Other callers keep the runtime-only contract unless they select this method.
/// (Python native và App/Flutter cần graph dev để test/build tools cùng nằm
/// trong lock MGC, không tự resolve ngoài.)
pub async fn resolve_with_protocol_including_dev(
    protocol: &dyn RegistryProtocol,
    ecosystem: EcosystemTag,
    registry: &str,
    manifest: &Manifest,
) -> MgResult<NativeResolution> {
    resolve_with_protocol_inner(protocol, ecosystem, registry, manifest, true).await
}

async fn resolve_with_protocol_inner(
    protocol: &dyn RegistryProtocol,
    ecosystem: EcosystemTag,
    registry: &str,
    manifest: &Manifest,
    include_dev_dependencies: bool,
) -> MgResult<NativeResolution> {
    // Resolve all selected roots in one protocol call so ecosystems with a
    // native multi-root solver can intersect constraints across runtime/dev
    // roots. The default protocol implementation still fails closed on
    // inconsistent per-root graphs.
    // (Gửi mọi root trong một lần để protocol có solver đa-root hợp nhất
    // constraint runtime/dev; protocol mặc định vẫn fail-closed khi lệch.)
    let mut roots = manifest
        .dependencies
        .iter()
        .map(|dependency| (dependency, false))
        .collect::<Vec<_>>();
    if include_dev_dependencies {
        roots.extend(
            manifest
                .dev_dependencies
                .iter()
                .map(|dependency| (dependency, true)),
        );
    }
    let root_specs = roots
        .iter()
        .map(|(dependency, _)| RegistryRoot {
            name: dependency.name.as_str().to_string(),
            range: dependency.range.as_str().to_string(),
            features: dependency.features.clone(),
            default_features: dependency.default_features,
        })
        .collect::<Vec<_>>();
    let entries = protocol
        .resolve_graph_roots_with_options(&root_specs)
        .await?;
    let chosen = entries
        .iter()
        .map(|entry| (entry_key(&entry.name, &entry.version), entry.clone()))
        .collect::<HashMap<String, ResolvedEntry>>();
    let mut by_name = BTreeMap::<String, Vec<String>>::new();
    for key in chosen.keys() {
        let (name, _) = key.rsplit_once('@').ok_or_else(|| {
            mgc_types::MgError::Other("resolved package identity is malformed".to_string())
        })?;
        by_name
            .entry(name.to_string())
            .or_default()
            .push(key.clone());
    }

    // Classify the resolved closure as dev-only only when no runtime root can
    // reach it. Shared transitive dependencies are runtime dependencies.
    // (Chỉ xếp dev-only khi không root runtime nào chạm tới; package dùng
    // chung giữa runtime/dev được coi là runtime.)
    let direct_ids = resolve_root_ids(protocol, &roots, &entries, &by_name)?;
    let runtime_roots = resolve_root_ids(
        protocol,
        &manifest
            .dependencies
            .iter()
            .map(|dependency| (dependency, false))
            .collect::<Vec<_>>(),
        &entries,
        &by_name,
    )?;
    let mut runtime_reachable = HashSet::new();
    let mut runtime_queue = runtime_roots.into_iter().collect::<VecDeque<_>>();
    while let Some(key) = runtime_queue.pop_front() {
        if !runtime_reachable.insert(key.clone()) {
            continue;
        }
        if let Some(entry) = chosen.get(&key) {
            for (dependency, range) in &entry.deps {
                runtime_queue.push_back(resolve_entry_key(dependency, range, &chosen, &by_name)?);
            }
        }
    }

    let mut packages = Vec::with_capacity(entries.len());
    let mut lock_packages = Vec::with_capacity(entries.len());
    for entry in &entries {
        let name = PackageName::new(entry.name.clone())
            .map_err(|_| mgc_types::MgError::InvalidPackageName(entry.name.clone()))?;
        let version = Version::parse(&entry.version)
            .map_err(|_| mgc_types::MgError::InvalidVersion(entry.version.clone()))?;
        let key = entry_key(&entry.name, &entry.version);
        let direct = direct_ids.contains(&key);

        let deps: Vec<PackageId> = entry
            .deps
            .iter()
            .map(|(dep_name, range)| {
                let target_key = resolve_entry_key(dep_name, range, &chosen, &by_name)?;
                let resolved = chosen.get(&target_key).ok_or_else(|| {
                    mgc_types::MgError::DependencyConflict(format!(
                        "resolved package {}@{} references missing dependency {} ({range})",
                        entry.name, entry.version, dep_name
                    ))
                })?;
                let package_name = PackageName::new(resolved.name.clone())
                    .map_err(|_| mgc_types::MgError::InvalidPackageName(dep_name.clone()))?;
                let dep_version = Version::parse(&resolved.version)
                    .map_err(|_| mgc_types::MgError::InvalidVersion(resolved.version.clone()))?;
                Ok(PackageId::new(package_name, dep_version))
            })
            .collect::<mgc_types::MgResult<Vec<_>>>()?;

        let integrity = resolved_integrity(ecosystem, entry)?;

        packages.push(ResolvedPackage {
            id: PackageId::new(name, version),
            integrity: integrity.clone(),
            tarball_url: entry.artifact_url.clone(),
            deps,
            peer_deps: Vec::new(),
            direct,
            dev: !runtime_reachable.contains(&key),
        });

        let lock_dependencies = entry
            .deps
            .iter()
            .map(|(name, range)| {
                let key = resolve_entry_key(name, range, &chosen, &by_name)?;
                Ok(if ecosystem == EcosystemTag::Rust {
                    key
                } else {
                    name.clone()
                })
            })
            .collect::<mgc_types::MgResult<Vec<_>>>()?;
        lock_packages.push(Package {
            owner_core: None,
            name: entry.name.clone(),
            version: entry.version.clone(),
            resolved: entry.artifact_url.clone(),
            integrity: integrity.clone(),
            dependencies: lock_dependencies,
            ecosystem,
            registry: Some(registry.to_string()),
            artifact: Some(ArtifactRef {
                url: entry.artifact_url.clone(),
                size_bytes: None,
                // blake3 filled at install (requires the downloaded bytes).
                content_hash: String::new(),
                downloaded_from: registry.to_string(),
                // NuGet's catalog SHA-512 is already verified by its protocol;
                // preserve it as installer-enforced SRI for V4 conversion.
                // SHA-512 catalog NuGet đã được protocol xác minh; giữ SRI để chuyển V4.
                integrity_sri: (ecosystem == EcosystemTag::NuGet).then(|| integrity.clone()),
            }),
            provenance: Some(Provenance {
                source_kind: SOURCE_KIND_NATIVE_RESOLVE.to_string(),
                tool: Some("mgc-resolver".to_string()),
                imported_from: None,
            }),
            markers: if entry.extra_markers.is_empty() {
                None
            } else {
                Some(entry.extra_markers.clone())
            },
            extras: None,
            peers: None,
            store_ref: None,
            toolchain: None,
            scripts_policy: None,
        });
    }

    Ok(NativeResolution {
        graph: ResolvedGraph { packages },
        lock_packages,
    })
}

fn entry_key(name: &str, version: &str) -> String {
    format!("{name}@{version}")
}

fn resolve_root_ids(
    protocol: &dyn RegistryProtocol,
    roots: &[(&mgc_types::DependencySpec, bool)],
    entries: &[ResolvedEntry],
    by_name: &BTreeMap<String, Vec<String>>,
) -> MgResult<HashSet<String>> {
    let mut selected_roots = HashSet::new();
    for (dependency, _) in roots {
        let name = dependency.name.as_str();
        let candidates = by_name.get(name).ok_or_else(|| {
            mgc_types::MgError::DependencyConflict(format!(
                "registry returned no resolved root package for '{name}'"
            ))
        })?;
        let mut matching = candidates
            .iter()
            .filter_map(|key| {
                let version = key.rsplit_once('@')?.1;
                let parsed = Version::parse(version).ok()?;
                protocol
                    .matches_requirement(dependency.range.as_str(), &parsed)
                    .then_some((parsed, key))
            })
            .collect::<Vec<_>>();
        matching.sort_by(|left, right| left.0.cmp(&right.0));
        let selected = matching.last().ok_or_else(|| {
            mgc_types::MgError::DependencyConflict(format!(
                "registry returned no version of '{name}' matching '{}'",
                dependency.range
            ))
        })?;
        selected_roots.insert(selected.1.clone());
    }
    let returned = entries
        .iter()
        .map(|entry| entry_key(&entry.name, &entry.version))
        .collect::<HashSet<_>>();
    if selected_roots.iter().any(|root| !returned.contains(root)) {
        return Err(mgc_types::MgError::Other(
            "registry root selection is absent from the resolved entries".to_string(),
        ));
    }
    Ok(selected_roots)
}

fn resolve_entry_key(
    name: &str,
    range: &str,
    entries: &HashMap<String, ResolvedEntry>,
    by_name: &BTreeMap<String, Vec<String>>,
) -> MgResult<String> {
    if let Some(version) = range.strip_prefix('=') {
        let key = entry_key(name, version);
        if entries.contains_key(&key) {
            return Ok(key);
        }
    }
    match by_name.get(name).map(Vec::as_slice).unwrap_or_default() {
        [key] => Ok(key.clone()),
        [] => Err(mgc_types::MgError::DependencyConflict(format!(
            "resolved graph is missing dependency '{name}' ({range})"
        ))),
        _ => Err(mgc_types::MgError::DependencyConflict(format!(
            "resolved dependency '{name}' is ambiguous without an exact version in range '{range}'"
        ))),
    }
}

/// Project registry-verified integrity into the shared graph representation.
/// Chuyển integrity registry đã xác minh sang biểu diễn graph dùng chung.
fn resolved_integrity(ecosystem: EcosystemTag, entry: &ResolvedEntry) -> MgResult<String> {
    if ecosystem != EcosystemTag::NuGet {
        return Ok(if entry.sha256.is_empty() {
            String::new()
        } else {
            format!("sha256-{}", entry.sha256)
        });
    }

    let hashes = entry
        .extra_markers
        .iter()
        .filter_map(|marker| marker.strip_prefix("sha512:"))
        .collect::<Vec<_>>();
    if hashes.len() != 1 {
        return Err(mgc_types::MgError::Integrity(format!(
            "NuGet resolution for {}@{} must contain exactly one SHA-512 registry marker",
            entry.name, entry.version
        )));
    }
    let digest = hashes[0];
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(digest)
        .map_err(|error| {
            mgc_types::MgError::Integrity(format!(
                "NuGet SHA-512 registry marker for {}@{} is not valid base64: {error}",
                entry.name, entry.version
            ))
        })?;
    if decoded.len() != 64 {
        return Err(mgc_types::MgError::Integrity(format!(
            "NuGet SHA-512 registry marker for {}@{} must decode to 64 bytes",
            entry.name, entry.version
        )));
    }
    Ok(format!("sha512-{digest}"))
}
