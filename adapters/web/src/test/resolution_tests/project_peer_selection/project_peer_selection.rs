// project_peer_selection behavior tests — test behavior thuộc owner project_peer_selection.
use super::*;

#[tokio::test]
async fn resolve_prefers_compatible_manifest_root_for_peer_dependency() {
    for (optional_range, include_optional) in [("*", true), ("^2.0.0", true), ("*", false)] {
        let shared = tempdir_real().unwrap();
        seed_shared_metadata(
            shared.path(),
            "typescript",
            serde_json::json!({
                "name": "typescript",
                "description": null,
                "versions": {
                    "6.0.3": {
                        "version": "6.0.3",
                        "dependencies": null,
                        "optionalDependencies": null,
                        "peerDependencies": null,
                        "peerDependenciesMeta": {},
                        "os": null,
                        "cpu": null,
                        "dist": {"tarball": "https://registry.example.test/typescript-6.0.3.tgz", "integrity": "sha512-ts6"}
                    },
                    "7.0.2": {
                        "version": "7.0.2",
                        "dependencies": null,
                        "optionalDependencies": null,
                        "peerDependencies": null,
                        "peerDependenciesMeta": {},
                        "os": null,
                        "cpu": null,
                        "dist": {"tarball": "https://registry.example.test/typescript-7.0.2.tgz", "integrity": "sha512-ts7"}
                    }
                },
                "dist-tags": {"latest": "7.0.2"},
                "time": seeded_publish_times(&serde_json::json!({"6.0.3": null, "7.0.2": null})
                    .as_object()
                    .expect("versions object")
                    .clone())
            }),
        );
        seed_shared_metadata(
            shared.path(),
            "vue-tsc-like",
            serde_json::json!({
                "name": "vue-tsc-like",
                "description": null,
                "versions": {
                    "1.0.0": {
                        "version": "1.0.0",
                        "dependencies": null,
                        "optionalDependencies": null,
                        "peerDependencies": {
                            "typescript": ">=5.0.0",
                            "optional-magic": optional_range
                        },
                        "peerDependenciesMeta": {
                            "optional-magic": {"optional": true}
                        },
                        "os": null,
                        "cpu": null,
                        "dist": {"tarball": "https://registry.example.test/vue-tsc-like-1.0.0.tgz", "integrity": "sha512-checker"}
                    }
                },
                "dist-tags": {"latest": "1.0.0"},
                "time": seeded_publish_times(&serde_json::json!({"1.0.0": null})
                    .as_object()
                    .expect("versions object")
                    .clone())
            }),
        );
        seed_shared_metadata(
            shared.path(),
            "ts7-consumer",
            serde_json::json!({
                "name": "ts7-consumer",
                "description": null,
                "versions": {
                    "1.0.0": {
                        "version": "1.0.0",
                        "dependencies": {"typescript": "^7.0.0"},
                        "optionalDependencies": null,
                        "peerDependencies": null,
                        "peerDependenciesMeta": {},
                        "os": null,
                        "cpu": null,
                        "dist": {"tarball": "https://registry.example.test/ts7-consumer-1.0.0.tgz", "integrity": "sha512-consumer"}
                    }
                },
                "dist-tags": {"latest": "1.0.0"},
                "time": seeded_publish_times(&serde_json::json!({"1.0.0": null})
                    .as_object()
                    .expect("versions object")
                    .clone())
            }),
        );
        seed_shared_metadata(
            shared.path(),
            "optional-magic",
            serde_json::json!({
                "name": "optional-magic",
                "description": null,
                "versions": {
                    "1.0.0": {
                        "version": "1.0.0",
                        "dependencies": null,
                        "optionalDependencies": null,
                        "peerDependencies": null,
                        "peerDependenciesMeta": {},
                        "os": null,
                        "cpu": null,
                        "dist": {"tarball": "https://registry.example.test/optional-magic-1.0.0.tgz", "integrity": "sha512-optional"}
                    }
                },
                "dist-tags": {"latest": "1.0.0"},
                "time": seeded_publish_times(&serde_json::json!({"1.0.0": null})
                    .as_object()
                    .expect("versions object")
                    .clone())
            }),
        );

        let adapter = WebAdapter::with_registry_and_shared_cache(
            "http://127.0.0.1:9".into(),
            shared.path().to_path_buf(),
        );
        let mut manifest = Manifest::new("demo", mgc_types::ecosystem::Ecosystem::Web);
        for (name, range) in [
            ("typescript", "^6.0.3"),
            ("vue-tsc-like", "^1.0.0"),
            ("ts7-consumer", "^1.0.0"),
            ("optional-magic", "^1.0.0"),
        ] {
            if name == "optional-magic" && !include_optional {
                continue;
            }
            manifest.add_dep(
                DependencySpec::new(
                    PackageName::new(name).unwrap(),
                    VersionRange::parse(range).unwrap(),
                ),
                true,
                false,
                false,
            );
        }

        let graph = with_project_dir(shared.path(), adapter.resolve_fresh(&manifest))
            .await
            .unwrap();
        let checker = graph
            .packages
            .iter()
            .find(|package| package.id.name_str() == "vue-tsc-like")
            .expect("test checker is present in the resolved graph");
        assert!(checker.deps.is_empty(), "peer packages use only peer edges");
        let mut peers = checker
            .peer_deps
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        peers.sort();
        let mut expected = vec!["typescript@6.0.3".to_string()];
        if include_optional && optional_range == "*" {
            expected.push("optional-magic@1.0.0".to_string());
        }
        expected.sort();
        assert_eq!(
            peers, expected,
            "provided optional peers must bind only when compatible"
        );
        assert_eq!(
            graph
                .packages
                .iter()
                .any(|package| package.id.name_str() == "optional-magic"),
            include_optional,
            "an absent optional peer must not be auto-installed"
        );
        assert!(
            graph
                .packages
                .iter()
                .any(|package| package.id.to_string() == "typescript@7.0.2"),
            "the newer transitive version remains available to its own consumer"
        );
    }
}

#[tokio::test]
async fn resolve_optional_peer_does_not_replace_declared_regular_dependency() {
    let shared = tempdir_real().unwrap();
    for (name, versions) in [
        (
            "peer-consumer",
            serde_json::json!({"1.0.0": {
                "version": "1.0.0",
                "dependencies": {"shared-runtime": "^1.0.0"},
                "peerDependencies": {"shared-runtime": "^2.0.0"},
                "peerDependenciesMeta": {"shared-runtime": {"optional": true}}
            }}),
        ),
        (
            "other-consumer",
            serde_json::json!({"1.0.0": {
                "version": "1.0.0", "dependencies": {"shared-runtime": "^2.0.0"}
            }}),
        ),
        (
            "shared-runtime",
            serde_json::json!({
                "1.0.0": {"version": "1.0.0"},
                "2.0.0": {"version": "2.0.0"}
            }),
        ),
    ] {
        seed_shared_metadata(
            shared.path(),
            name,
            serde_json::json!({
                "name": name, "versions": versions, "dist-tags": {"latest": "1.0.0"},
                "time": seeded_publish_times(versions.as_object().expect("versions object"))
            }),
        );
    }
    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
    let mut manifest = Manifest::new("peer-precedence", mgc_types::ecosystem::Ecosystem::Web);
    for name in ["peer-consumer", "other-consumer"] {
        manifest.add_dep(
            DependencySpec::new(
                PackageName::new(name).unwrap(),
                VersionRange::parse("^1.0.0").unwrap(),
            ),
            true,
            false,
            false,
        );
    }
    let graph = with_project_dir(shared.path(), adapter.resolve_fresh(&manifest))
        .await
        .unwrap();
    let consumer = graph
        .packages
        .iter()
        .find(|package| package.id.name_str() == "peer-consumer")
        .unwrap();
    assert_eq!(
        consumer.deps,
        vec![PackageId::parse("shared-runtime@1.0.0").unwrap()]
    );
    assert!(
        consumer.peer_deps.is_empty(),
        "Optional peer inference must not replace the regular dependency with a different graph version"
    );
    assert!(
        graph
            .packages
            .iter()
            .any(|package| package.id.to_string() == "shared-runtime@2.0.0")
    );
}
