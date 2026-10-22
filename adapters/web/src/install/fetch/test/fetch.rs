#![allow(clippy::unwrap_used)]
//! Fetch coordinator contract tests. / Kiểm thử contract coordinator fetch.

use super::super::*;
use base64::Engine as _;
use mgc_types::{PackageId, adapter::ResolvedPackage};
use sha2::{Digest, Sha512};

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
