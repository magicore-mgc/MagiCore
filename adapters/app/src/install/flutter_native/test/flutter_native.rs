//! Tests for complete Flutter archive cache preflight.
//! Test tiền kiểm tra toàn bộ cache archive Flutter.

use super::*;

#[test]
fn flutter_offline_preflight_checks_every_archive_without_materialization() {
    use sha2::{Digest, Sha256};
    let cache = tempfile::tempdir().unwrap();
    let bytes = b"first verified archive";
    let digest = hex::encode(Sha256::digest(bytes));
    super::pub_archive_cache::save_verified(cache.path(), &digest, bytes).unwrap();
    let entry = |name: &str, sha: &str| ResolvedPackage {
        id: PackageId::new(
            PackageName::new(name).unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: format!("sha256-{sha}"),
        tarball_url: format!("https://pub.dev/{name}.tar.gz"),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    let graph = ResolvedGraph {
        packages: vec![entry("first", &digest), entry("missing", &"0".repeat(64))],
    };
    let error = super::preflight_flutter_archives(&graph, cache.path()).unwrap_err();
    assert!(error.to_string().contains("cache miss"));
    assert!(!cache.path().join("hosted").exists());
    let graph = ResolvedGraph {
        packages: vec![entry("first", &digest)],
    };
    super::preflight_flutter_archives(&graph, cache.path()).unwrap();
    assert!(!cache.path().join("hosted").exists());
    std::fs::write(
        cache
            .path()
            .join("archives")
            .join(format!("{digest}.tar.gz")),
        b"tampered",
    )
    .unwrap();
    assert!(super::preflight_flutter_archives(&graph, cache.path()).is_err());
}
