//! Shared-resolution cache behavior tests.
//! Test hành vi cache resolution dùng chung.

use super::support::tempdir_real;
use crate::WebAdapter;
use crate::cache::SharedWebCache;
use crate::manifest_resolution_cache_key;
use crate::provider::AgePolicy;
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};
use mgc_types::capabilities::DependencyResolver;
use mgc_types::{DependencySpec, Manifest, PackageId, PackageName, Version, VersionRange};

#[tokio::test]
async fn test_resolve_uses_shared_resolution_cache_when_registry_is_unavailable() {
    let shared = tempdir_real().unwrap();
    let registry_url = "http://127.0.0.1:9";
    let cache = SharedWebCache {
        root: shared.path().to_path_buf(),
    };
    let mut manifest = Manifest::new("demo-a", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^18.2.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("react").unwrap(),
                Version::parse("18.2.0").unwrap(),
            ),
            integrity: "sha512-react".to_string(),
            tarball_url: "https://registry.example.test/react-18.2.0.tgz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let key = manifest_resolution_cache_key(&manifest, registry_url);
    cache.write_resolution(&key, registry_url, &graph).unwrap();

    let adapter = WebAdapter::with_registry_and_shared_cache(
        registry_url.to_string(),
        shared.path().to_path_buf(),
    );
    let resolved = adapter.resolve(&manifest).await.unwrap();

    assert_eq!(resolved.packages.len(), 1);
    assert_eq!(resolved.packages[0].id.to_string(), "react@18.2.0");
    assert_eq!(resolved.packages[0].integrity, "sha512-react");
}

#[tokio::test]
async fn test_resolve_bypasses_shared_cache_when_minimum_age_is_active() {
    let shared = tempdir_real().unwrap();
    let registry_url = "http://127.0.0.1:9";
    let cache = SharedWebCache {
        root: shared.path().to_path_buf(),
    };
    let mut manifest = Manifest::new("demo-age", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^18.2.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("react").unwrap(),
                Version::parse("18.2.0").unwrap(),
            ),
            integrity: "sha512-stale-age-unverified".to_string(),
            tarball_url: "https://registry.example.test/react-18.2.0.tgz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let key = manifest_resolution_cache_key(&manifest, registry_url);
    cache.write_resolution(&key, registry_url, &graph).unwrap();

    let adapter = WebAdapter::with_registry_and_shared_cache(
        registry_url.to_string(),
        shared.path().to_path_buf(),
    );
    adapter.provider.set_age_policy(Some(AgePolicy {
        cutoff_hours: 24,
        allow_missing_time: false,
    }));

    let error = adapter.resolve(&manifest).await.unwrap_err();
    assert!(
        error.to_string().contains("127.0.0.1:9") || error.to_string().contains("offline mode"),
        "active age gate must not reuse an unverified cached graph: {error}"
    );
}

#[tokio::test]
async fn test_resolve_does_not_reuse_legacy_resolution_cache_schema() {
    use sha2::{Digest, Sha256};

    let shared = tempdir_real().unwrap();
    let registry_url = "http://127.0.0.1:9";
    let cache = SharedWebCache {
        root: shared.path().to_path_buf(),
    };
    let mut manifest = Manifest::new("demo", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("react").unwrap(),
            VersionRange::parse("^18.2.0").unwrap(),
        ),
        false,
        false,
        false,
    );

    let mut entries = Vec::new();
    for (group, deps) in manifest.dep_groups() {
        for dep in deps {
            entries.push(format!(
                "{}\0{}\0{}\0{}\0{}\0{}",
                group,
                dep.name.as_str(),
                dep.range.as_str(),
                dep.dev,
                dep.optional,
                dep.peer
            ));
        }
    }
    entries.sort_unstable();
    let stale_graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("react").unwrap(),
                Version::parse("18.2.0").unwrap(),
            ),
            integrity: "sha512-legacy-cache".to_string(),
            tarball_url: "https://registry.example.test/legacy-react.tgz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    for legacy_domain in [
        b"magicore-web-resolution-v1\0".as_slice(),
        b"magicore-web-resolution-v2\0".as_slice(),
    ] {
        let mut hasher = Sha256::new();
        hasher.update(legacy_domain);
        hasher.update(registry_url.as_bytes());
        hasher.update(b"\0");
        for entry in &entries {
            hasher.update(entry.as_bytes());
            hasher.update(b"\0");
        }
        let legacy_key = hex::encode(hasher.finalize());
        cache
            .write_resolution(&legacy_key, registry_url, &stale_graph)
            .unwrap();
    }

    let adapter = WebAdapter::with_registry_and_shared_cache(
        registry_url.to_string(),
        shared.path().to_path_buf(),
    );
    let result = adapter.resolve(&manifest).await;
    assert!(
        result.is_err(),
        "legacy resolver cache must be invalidated after graph semantics change"
    );
}
