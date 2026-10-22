// alias_dependencies behavior tests — test behavior thuộc owner alias_dependencies.
use super::*;

#[tokio::test]
async fn test_alias_dependency_uses_target_metadata_and_range() {
    let shared = tempdir_real().unwrap();

    seed_shared_metadata(
        shared.path(),
        "demo-parent",
        serde_json::json!({
            "name": "demo-parent",
            "description": null,
            "versions": {
                "1.0.0": {
                    "version": "1.0.0",
                    "dependencies": {
                        "strip-ansi-cjs": "npm:strip-ansi@^6.0.1"
                    },
                    "optionalDependencies": null,
                    "os": null,
                    "cpu": null,
                    "dist": {
                        "tarball": "https://registry.example.test/demo-parent-1.0.0.tgz",
                        "integrity": "sha512-parent"
                    }
                }
            },
            "dist-tags": { "latest": "1.0.0" }
        }),
    );
    seed_shared_metadata(
        shared.path(),
        "strip-ansi",
        serde_json::json!({
            "name": "strip-ansi",
            "description": null,
            "versions": {
                "6.0.1": {
                    "version": "6.0.1",
                    "dependencies": null,
                    "optionalDependencies": null,
                    "os": null,
                    "cpu": null,
                    "dist": {
                        "tarball": "https://registry.example.test/strip-ansi-6.0.1.tgz",
                        "integrity": "sha512-strip-ansi"
                    }
                }
            },
            "dist-tags": { "latest": "6.0.1" }
        }),
    );

    let provider = NpmDependencyProvider::new(
        "http://127.0.0.1:9",
        None,
        Some(SharedWebCache {
            root: shared.path().to_path_buf(),
        }),
    );
    let parent_id = PackageId::new(
        PackageName::new("demo-parent").unwrap(),
        Version::parse("1.0.0").unwrap(),
    );

    let deps = provider.get_dependencies(&parent_id).await.unwrap();
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].package.as_str(), "strip-ansi-cjs");
    assert_eq!(deps[0].spec, "^6.0.1");

    let versions = provider.get_versions(&deps[0].package).await.unwrap();
    assert!(
        versions
            .iter()
            .any(|version| version.to_string() == "6.0.1")
    );
}

#[tokio::test]
async fn npm_optional_peer_dependencies_are_not_auto_installed() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind local registry for peer metadata regression");
    let addr = listener.local_addr().unwrap();
    let body = r#"{
        "name":"angular-build-like",
        "versions":{"1.0.0":{
            "version":"1.0.0",
            "dependencies":{"required-runtime":"^1.0.0"},
            "peerDependencies":{
                "optional-browser":"^5.0.0",
                "required-plugin":"^2.0.0"
            },
            "peerDependenciesMeta":{"optional-browser":{"optional":true}}
        }},
        "dist-tags":{"latest":"1.0.0"}
    }"#;
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = [0u8; 4096];
        let _ = stream.read(&mut request).await.unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).await.unwrap();
    });

    let provider = NpmDependencyProvider::new(&format!("http://{addr}"), None, None);
    let package_id = PackageId::new(
        PackageName::new("angular-build-like").unwrap(),
        Version::parse("1.0.0").unwrap(),
    );
    let mut names: Vec<_> = provider
        .get_dependencies(&package_id)
        .await
        .unwrap()
        .into_iter()
        .map(|dependency| dependency.package.to_string())
        .collect();
    names.sort();
    assert_eq!(names, ["required-plugin", "required-runtime"]);
    server.await.unwrap();
}

