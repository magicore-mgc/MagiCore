//! Exercises the complete online install path with a local registry fixture.
//! Chạy toàn luồng install online với fixture registry cục bộ.

use super::super::install_rust_online_with_protocol;
use crate::install::rust_native_install::test::{resolved_rust, test_crate_archive};
use mgc_resolver::protocols::CratesProtocol;
use mgc_store::{ContentStore, IntegrityHash};
use mgc_types::ResolvedGraph;

#[tokio::test]
async fn downloads_verifies_imports_and_materializes_from_local_registry() {
    let archive = test_crate_archive("mgcfixture", "1.0.0");
    let mut server = mockito::Server::new_async().await;
    let archive_url = format!("{}/mgcfixture-1.0.0.crate", server.url());
    let request = server
        .mock("GET", "/mgcfixture-1.0.0.crate")
        .with_status(200)
        .with_body(archive.clone())
        .create_async()
        .await;
    let mut package = resolved_rust("mgcfixture", "1.0.0", &archive);
    package.tarball_url = archive_url;
    let graph = ResolvedGraph {
        packages: vec![package],
    };
    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let cargo_home = tempfile::tempdir().unwrap();
    let cas_root = tempfile::tempdir().unwrap();
    let store = ContentStore::new(cas_root.path().to_path_buf()).unwrap();

    let summary = install_rust_online_with_protocol(
        &graph,
        cargo_home.path(),
        &protocol,
        &store,
        std::time::Instant::now(),
    )
    .await
    .unwrap();

    request.assert_async().await;
    assert_eq!(summary.added.len(), 1);
    assert_eq!(summary.bytes_from_cache, 0);
    assert!(
        cargo_home
            .path()
            .join("registry/cache/mgcfixture-1.0.0.crate")
            .is_file()
    );
    let source_root = cargo_home.path().join("registry/src/mgcfixture-1.0.0");
    assert!(source_root.join("Cargo.toml").is_file());
    assert!(source_root.join(".cargo-checksum.json").is_file());
    let hash = IntegrityHash::from_bytes(&archive, false);
    assert!(store.contains(&hash));
}

#[tokio::test]
async fn uses_verified_cargo_cache_without_network_and_counts_cached_bytes() {
    let archive = test_crate_archive("mgcfixture", "1.0.0");
    let mut server = mockito::Server::new_async().await;
    let cache_hit_request = server
        .mock("GET", "/must-not-download.crate")
        .expect(0)
        .create_async()
        .await;
    let mut package = resolved_rust("mgcfixture", "1.0.0", &archive);
    package.tarball_url = format!("{}/must-not-download.crate", server.url());
    let graph = ResolvedGraph {
        packages: vec![package],
    };
    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let cargo_home = tempfile::tempdir().unwrap();
    let cached_archive = cargo_home
        .path()
        .join("registry/cache/mgcfixture-1.0.0.crate");
    std::fs::create_dir_all(cached_archive.parent().unwrap()).unwrap();
    std::fs::write(&cached_archive, &archive).unwrap();
    let cas_root = tempfile::tempdir().unwrap();
    let store = ContentStore::new(cas_root.path().to_path_buf()).unwrap();

    let summary = install_rust_online_with_protocol(
        &graph,
        cargo_home.path(),
        &protocol,
        &store,
        std::time::Instant::now(),
    )
    .await
    .unwrap();

    cache_hit_request.assert_async().await;
    assert_eq!(summary.added.len(), 1);
    assert_eq!(summary.bytes_from_cache, archive.len() as u64);
    assert!(
        cargo_home
            .path()
            .join("registry/src/mgcfixture-1.0.0/.cargo-checksum.json")
            .is_file()
    );
}

#[tokio::test]
async fn rejects_registry_checksum_mismatch_before_cas_import_or_materialization() {
    let archive = test_crate_archive("mgcfixture", "1.0.0");
    let mut server = mockito::Server::new_async().await;
    let archive_url = format!("{}/mgcfixture-1.0.0.crate", server.url());
    let request = server
        .mock("GET", "/mgcfixture-1.0.0.crate")
        .with_status(200)
        .with_body(archive.clone())
        .create_async()
        .await;
    let mut package = resolved_rust("mgcfixture", "1.0.0", &archive);
    package.tarball_url = archive_url;
    package.integrity = format!("sha256-{}", "00".repeat(32));
    let graph = ResolvedGraph {
        packages: vec![package],
    };
    let protocol = CratesProtocol::with_download_base(&server.url(), &server.url());
    let cargo_home = tempfile::tempdir().unwrap();
    let cas_root = tempfile::tempdir().unwrap();
    let store = ContentStore::new(cas_root.path().to_path_buf()).unwrap();

    let error = install_rust_online_with_protocol(
        &graph,
        cargo_home.path(),
        &protocol,
        &store,
        std::time::Instant::now(),
    )
    .await
    .unwrap_err();

    request.assert_async().await;
    assert!(error.to_string().contains("sha256 mismatch"));
    assert!(
        !cargo_home
            .path()
            .join("registry/cache/mgcfixture-1.0.0.crate")
            .exists()
    );
    assert!(
        !cargo_home
            .path()
            .join("registry/src/mgcfixture-1.0.0")
            .exists()
    );
    assert!(!store.contains(&IntegrityHash::from_bytes(&archive, false)));
}
