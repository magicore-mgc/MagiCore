//! Tests offline cache replay and preflight-before-write behavior.
//! Kiểm tra replay cache offline và preflight trước khi ghi.

use crate::install::rust_native_install::test::{resolved_rust, test_crate_archive};
use crate::install::rust_native_install::{install_rust_cached, preflight_rust_artifact_cache};
use mgc_types::ResolvedGraph;

#[test]
fn rust_offline_cache_is_sha256_verified_and_rematerialized() {
    let archive = test_crate_archive("mgcfixture", "1.0.0");
    let graph = ResolvedGraph {
        packages: vec![resolved_rust("mgcfixture", "1.0.0", &archive)],
    };
    let cargo_home = tempfile::tempdir().unwrap();
    let archive_path = cargo_home
        .path()
        .join("registry/cache/mgcfixture-1.0.0.crate");
    std::fs::create_dir_all(archive_path.parent().unwrap()).unwrap();
    std::fs::write(&archive_path, &archive).unwrap();
    preflight_rust_artifact_cache(&graph, cargo_home.path()).unwrap();

    let cas_root = tempfile::tempdir().unwrap();
    let store = mgc_store::ContentStore::new(cas_root.path().to_path_buf()).unwrap();
    let summary =
        install_rust_cached(&graph, cargo_home.path(), &store, std::time::Instant::now()).unwrap();

    assert_eq!(summary.added.len(), 1);
    assert_eq!(summary.bytes_from_cache, archive.len() as u64);
    assert_eq!(std::fs::read(&archive_path).unwrap(), archive);
    assert!(
        cargo_home
            .path()
            .join("registry/src/mgcfixture-1.0.0/Cargo.toml")
            .is_file()
    );
}

#[test]
fn rust_offline_cache_preflight_fails_before_materializing_partial_graph() {
    let first_archive = test_crate_archive("firstcrate", "1.0.0");
    let second_archive = test_crate_archive("secondcrate", "1.0.0");
    let graph = ResolvedGraph {
        packages: vec![
            resolved_rust("firstcrate", "1.0.0", &first_archive),
            resolved_rust("secondcrate", "1.0.0", &second_archive),
        ],
    };
    let cargo_home = tempfile::tempdir().unwrap();
    let first_path = cargo_home
        .path()
        .join("registry/cache/firstcrate-1.0.0.crate");
    std::fs::create_dir_all(first_path.parent().unwrap()).unwrap();
    std::fs::write(&first_path, &first_archive).unwrap();

    let error = preflight_rust_artifact_cache(&graph, cargo_home.path()).unwrap_err();
    assert!(
        error.to_string().contains("secondcrate") && error.to_string().contains("1.0.0"),
        "unexpected preflight error: {error:#}"
    );
    assert!(
        !cargo_home
            .path()
            .join("registry/src/firstcrate-1.0.0")
            .exists()
    );
}
