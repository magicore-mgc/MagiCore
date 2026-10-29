#![allow(clippy::unwrap_used)]
//! Fetch coordinator contract tests. / Kiểm thử contract coordinator fetch.

use super::super::*;
use base64::Engine as _;
use mgc_types::{PackageId, adapter::ResolvedPackage};
use sha2::{Digest, Sha512};

/// SEC-01 regression: a cache entry written for one registry must never be
/// served to a project resolving the same `name@version` from another
/// registry, even when the second lock entry carries no SRI evidence.
/// Hồi quy SEC-01: entry cache của registry này không bao giờ được phục vụ
/// cho registry khác dù lock không có SRI.
#[tokio::test]
async fn sec01_cached_tarball_from_another_registry_is_not_served() {
    let root = tempfile::tempdir().unwrap();
    let registry_x =
        PackageCache::for_registry(root.path().join("package-cache"), "https://reg-x.example/")
            .unwrap();
    let id = PackageId::parse("poisoned-pkg@1.0.0").unwrap();
    let poisoned = b"bytes published by registry X";
    registry_x.cache_tarball(&id, poisoned).unwrap();

    let registry_y =
        PackageCache::for_registry(root.path().join("package-cache"), "https://reg-y.example/")
            .unwrap();
    assert!(
        !registry_y.contains_tarball(&id),
        "registry Y must not see registry X's tarball"
    );
    assert!(
        registry_y.open_tarball(&id).unwrap().is_none(),
        "registry Y must not open registry X's tarball"
    );

    let package = ResolvedPackage {
        id: id.clone(),
        integrity: String::new(),
        tarball_url: "https://reg-y.example/poisoned-pkg-1.0.0.tgz".into(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    };
    let download_semaphore = tokio::sync::Semaphore::new(1);
    let result =
        get_tarball_bytes_with_registry(&package, &registry_y, None, None, &download_semaphore)
            .await;
    assert!(
        result.is_err(),
        "without a registry client the fetch must not fall back to another registry's cache"
    );
}

#[tokio::test]
async fn valid_local_cache_hit_does_not_require_registry() {
    let root = tempfile::tempdir().unwrap();
    let cache = PackageCache::new(root.path().join("package-cache")).unwrap();
    let id = PackageId::parse("cached-package@1.0.0").unwrap();
    let bytes = b"already cached tarball";
    let integrity = format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes))
    );
    cache.cache_tarball(&id, bytes).unwrap();
    let package = ResolvedPackage {
        id,
        integrity,
        tarball_url: String::new(),
        deps: Vec::new(),
        peer_deps: Vec::new(),
        direct: true,
        dev: false,
    };
    let download_semaphore = tokio::sync::Semaphore::new(1);

    let result = get_tarball_bytes_with_registry(&package, &cache, None, None, &download_semaphore)
        .await
        .unwrap();

    match result.payload {
        TarballPayload::CachedPath(path, len) => {
            assert_eq!(len, bytes.len() as u64);
            assert_eq!(std::fs::read(path).unwrap(), bytes);
        }
        TarballPayload::Bytes(_) => panic!("a local cache hit should stay file-backed"),
    }
    assert_eq!(result.queue_wait_ms, 0);
    assert!(!result.persist_to_shared_cache);
}
