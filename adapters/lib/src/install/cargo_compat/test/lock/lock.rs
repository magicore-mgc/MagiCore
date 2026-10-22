use super::super::*;

#[tokio::test]
async fn rust_feature_graph_validation_accepts_feature_table_without_lock_writes() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nfixture = { version = \"1\", features = [\"derive\"] }\n",
    )
    .unwrap();
    let archive = crate_archive("fixture", "1.0.0");
    let checksum = hex::encode(sha2::Sha256::digest(&archive));
    let package = locked_package("fixture", &[]);
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("fixture").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ),
            integrity: format!("sha256-{checksum}"),
            tarball_url: "http://127.0.0.1:1/fixture.crate".to_string(),
            deps: Vec::new(),
            peer_deps: Vec::new(),
            direct: true,
            dev: false,
        }],
    };

    validate_install(project.path(), &[package], &graph).unwrap();
    assert_eq!(graph.packages.len(), 1);
    assert!(!project.path().join("mgc.lock").exists());
    assert!(!project.path().join("Cargo.lock").exists());
}
