// metadata_resolution behavior tests — test behavior thuộc owner metadata_resolution.
use super::*;

#[tokio::test]
async fn test_resolve_populates_tarball_url_and_integrity_from_shared_metadata() {
    let shared = tempdir_real().unwrap();

    seed_shared_metadata(
        shared.path(),
        "react",
        serde_json::json!({
            "name": "react",
            "description": null,
            "versions": {
                "18.2.0": {
                    "version": "18.2.0",
                    "dependencies": null,
                    "optionalDependencies": null,
                    "os": null,
                    "cpu": null,
                    "dist": {
                        "tarball": "https://registry.example.test/react-18.2.0.tgz",
                        "integrity": "sha512-c2hhcmVk"
                    }
                }
            },
            "dist-tags": {
                "latest": "18.2.0"
            }
        }),
    );

    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
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

    let graph = adapter.resolve(&manifest).await.unwrap();
    assert_eq!(graph.packages.len(), 1);
    assert_eq!(
        graph.packages[0].tarball_url,
        "https://registry.example.test/react-18.2.0.tgz"
    );
    assert_eq!(graph.packages[0].integrity, "sha512-c2hhcmVk");
}

