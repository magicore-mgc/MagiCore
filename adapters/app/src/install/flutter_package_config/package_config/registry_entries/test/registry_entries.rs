//! Tests for verified pub.dev package identity in generated configuration.
//! Test danh tính package pub.dev đã xác minh trong config được tạo.

use super::*;

#[test]
fn flutter_package_config_rejects_archive_identity_mismatch() {
    let project = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("pubspec.yaml"), "name: sample\n").unwrap();
    let package_root = cache.path().join("hosted/pub.dev/http-1.2.0");
    std::fs::create_dir_all(&package_root).unwrap();
    std::fs::write(package_root.join("pubspec.yaml"), "name: different\n").unwrap();
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("http").unwrap(),
                Version::parse("1.2.0").unwrap(),
            ),
            integrity: "sha256-fixture".to_string(),
            tarball_url: "https://pub.dev/http.tar.gz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    let error = write_flutter_package_config(&graph, project.path(), cache.path()).unwrap_err();
    assert!(error.to_string().contains("identity mismatch"));
    assert!(
        !project
            .path()
            .join(".dart_tool/package_config.json")
            .exists()
    );
}
