// dist_tag_resolution behavior tests — test behavior thuộc owner dist_tag_resolution.
use super::*;

#[test]
fn is_dist_tag_spec_tags_vs_semver() {
    // Tag registry (chỉ chữ cái) — đi đường packument, không phải semver
    assert!(WebAdapter::is_dist_tag_spec("latest"));
    assert!(WebAdapter::is_dist_tag_spec("next"));
    assert!(WebAdapter::is_dist_tag_spec("beta"));
    // Semver/range/rỗng — giữ đường semver cũ (hành vi + lỗi không đổi)
    assert!(!WebAdapter::is_dist_tag_spec("*"));
    assert!(!WebAdapter::is_dist_tag_spec(""));
    assert!(!WebAdapter::is_dist_tag_spec("^1.2.3"));
    assert!(!WebAdapter::is_dist_tag_spec("1.2.3"));
    assert!(!WebAdapter::is_dist_tag_spec(">=1.0.0 <2.0.0"));
    assert!(!WebAdapter::is_dist_tag_spec("abc123"));
}

#[tokio::test]
async fn web_resolve_keeps_dist_tag_root_and_refreshes_when_registry_tag_moves() {
    let shared = tempdir_real().unwrap();
    let package_metadata = |latest: &str| {
        serde_json::json!({
            "name": "tagged-tool",
            "description": null,
            "versions": {
                "1.0.0": {
                    "version": "1.0.0",
                    "dependencies": null,
                    "optionalDependencies": null,
                    "os": null,
                    "cpu": null,
                    "dist": {
                        "tarball": "https://registry.example.test/tagged-tool-1.tgz",
                        "integrity": "sha512-c2hhcmVk"
                    }
                },
                "2.0.0": {
                    "version": "2.0.0",
                    "dependencies": null,
                    "optionalDependencies": null,
                    "os": null,
                    "cpu": null,
                    "dist": {
                        "tarball": "https://registry.example.test/tagged-tool-2.tgz",
                        "integrity": "sha512-c2hhcmVk"
                    }
                }
            },
            "dist-tags": { "latest": latest }
        })
    };
    let manifest = || {
        let mut manifest =
            Manifest::new("tagged-tool-consumer", mgc_types::ecosystem::Ecosystem::Web);
        manifest.add_dep(
            DependencySpec::new(
                PackageName::new("tagged-tool").unwrap(),
                VersionRange::parse("latest").unwrap(),
            ),
            false,
            false,
            false,
        );
        manifest
    };
    let resolve = || async {
        let adapter = WebAdapter::with_registry_and_shared_cache(
            "http://127.0.0.1:9".into(),
            shared.path().to_path_buf(),
        );
        adapter.resolve(&manifest()).await.unwrap()
    };

    seed_shared_metadata(shared.path(), "tagged-tool", package_metadata("1.0.0"));
    let first = resolve().await;
    assert!(first.packages.iter().any(|package| {
        package.id.name_str() == "tagged-tool"
            && package.id.version().to_string() == "1.0.0"
            && package.direct
    }));

    seed_shared_metadata(shared.path(), "tagged-tool", package_metadata("2.0.0"));
    let second = resolve().await;
    assert!(second.packages.iter().any(|package| {
        package.id.name_str() == "tagged-tool"
            && package.id.version().to_string() == "2.0.0"
            && package.direct
    }));
}
