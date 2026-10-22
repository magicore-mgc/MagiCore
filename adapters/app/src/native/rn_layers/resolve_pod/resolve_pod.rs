//! Verify one podspec against Podfile.lock before projecting graph and lock entries.
//! Xác minh một podspec theo Podfile.lock trước khi tạo graph và lock entry.

use mgc_lockfile::{EcosystemTag, Package};
use mgc_resolver::protocols::reactnative::{
    CocoaPodsProtocol, PODSPEC_SHA1_MARKER_PREFIX, PodfileLock, PodfileLockPod,
    RN_TIER_MARKER_PREFIX, cocoapods_spec_url, pod_root,
};
use mgc_types::{MgError, MgResult, PackageId, PackageName, ResolvedPackage, Version};

/// A single verified pod's graph node and lock record.
/// Graph node và lock record của một pod đã xác minh.
pub(in crate::native::rn_layers) struct ResolvedPod {
    pub(in crate::native::rn_layers) graph_package: ResolvedPackage,
    pub(in crate::native::rn_layers) lock_package: Package,
}

/// Fetch and verify one podspec, then build its graph and lock projection.
/// Tải/xác minh một podspec rồi tạo graph và lock projection.
pub(in crate::native::rn_layers) async fn resolve_pod(
    pod: &PodfileLockPod,
    lock: &PodfileLock,
    cocoapods: &CocoaPodsProtocol,
) -> MgResult<ResolvedPod> {
    let Some(expected) = lock.checksum_for(&pod.name) else {
        return Err(MgError::Integrity(format!(
            "Podfile.lock has no SPEC CHECKSUMS entry for pod '{}' — the spec cannot be verified (fail-closed)",
            pod.name
        )));
    };
    let verified = cocoapods
        .fetch_verified_spec(&pod.name, &pod.version, expected)
        .await?;
    let dep_names: Vec<String> = CocoaPodsProtocol::spec_dependencies(&verified.spec)
        .into_iter()
        .map(|(dependency, _)| pod_root(&dependency).to_string())
        .filter(|root| lock.pods.iter().any(|locked| &locked.name == root))
        .collect();
    let spec_url = cocoapods_spec_url(cocoapods.cdn(), &pod.name, &pod.version);
    let id = PackageId::new(
        PackageName::new(pod.name.clone())
            .map_err(|e| MgError::Other(format!("invalid pod name {}: {e}", pod.name)))?,
        Version::parse(&pod.version)
            .map_err(|e| MgError::Other(format!("invalid pod version {}: {e}", pod.version)))?,
    );
    let graph_package = ResolvedPackage {
        id,
        integrity: String::new(),
        tarball_url: spec_url.clone(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: false,
        dev: false,
    };
    let markers = vec![
        format!("{PODSPEC_SHA1_MARKER_PREFIX}{}", verified.sha1),
        format!("{RN_TIER_MARKER_PREFIX}ios"),
        format!("checked-deps:{}", dep_names.len()),
    ];
    let lock_package = Package {
        owner_core: Some("app".to_string()),
        name: pod.name.clone(),
        version: pod.version.clone(),
        resolved: spec_url.clone(),
        integrity: String::new(),
        dependencies: dep_names,
        ecosystem: EcosystemTag::CocoaPods,
        registry: Some(
            crate::native::rn_layers::lock_markers::cocoapods_registry_tag(cocoapods.cdn()),
        ),
        artifact: Some(mgc_lockfile::ArtifactRef {
            url: spec_url,
            size_bytes: None,
            content_hash: String::new(),
            downloaded_from: crate::native::rn_layers::lock_markers::cocoapods_registry_tag(
                cocoapods.cdn(),
            ),
            integrity_sri: None,
        }),
        provenance: Some(mgc_lockfile::Provenance {
            source_kind: mgc_lockfile::SOURCE_KIND_NATIVE_RESOLVE.to_string(),
            tool: Some("mgc-resolver".to_string()),
            imported_from: None,
        }),
        markers: Some(markers),
        ..Default::default()
    };
    Ok(ResolvedPod {
        graph_package,
        lock_package,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
