// platform_alias behavior tests — test behavior thuộc owner platform_alias.
use super::*;

#[tokio::test]
async fn npm_optional_platform_alias_resolves_its_prerelease_version() {
    let shared = tempdir_real().unwrap();
    let mut optional = serde_json::Map::new();
    let mut versions = serde_json::Map::new();
    for (os, cpu) in [
        ("darwin", "x64"),
        ("darwin", "arm64"),
        ("linux", "x64"),
        ("linux", "arm64"),
        ("win32", "x64"),
        ("win32", "arm64"),
    ] {
        let alias = format!("@openai/codex-{os}-{cpu}");
        let platform_version = format!("0.162.0-{os}-{cpu}");
        optional.insert(
            alias,
            serde_json::Value::String(format!("npm:@openai/codex@{platform_version}")),
        );
        versions.insert(
            platform_version.clone(),
            serde_json::json!({
                "version": platform_version,
                "dependencies": null,
                "optionalDependencies": null,
                "os": [os],
                "cpu": [cpu],
                "dist": {
                    "tarball": format!("https://registry.example.test/codex-{os}-{cpu}.tgz"),
                    "integrity": "sha512-platform"
                }
            }),
        );
    }
    versions.insert(
        "0.162.0".to_string(),
        serde_json::json!({
            "version": "0.162.0",
            "dependencies": null,
            "optionalDependencies": optional,
            "os": null,
            "cpu": null,
            "dist": {
                "tarball": "https://registry.example.test/codex.tgz",
                "integrity": "sha512-wrapper"
            }
        }),
    );
    seed_shared_metadata(
        shared.path(),
        "@openai/codex",
        serde_json::json!({
            "name": "@openai/codex",
            "description": null,
            "versions": versions,
            "dist-tags": { "latest": "0.162.0" }
        }),
    );

    let provider = NpmDependencyProvider::new(
        "http://127.0.0.1:9",
        None,
        Some(SharedWebCache {
            root: shared.path().to_path_buf(),
        }),
    );
    let resolver = mgc_resolver::Resolver::new(Arc::new(provider));
    let result = resolver
        .solve(&[(
            PackageName::new("@openai/codex").unwrap(),
            "0.162.0".to_string(),
        )])
        .await
        .unwrap();
    let platform_package = format!(
        "@openai/codex-{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    );

    assert!(
        result
            .resolutions
            .iter()
            .any(|resolution| resolution.package_id.name_str() == platform_package),
        "platform-specific optional alias was not resolved: {:?}",
        result
            .resolutions
            .iter()
            .map(|resolution| resolution.package_id.to_string())
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn web_graph_keeps_codex_platform_alias_reachable() {
    let shared = tempdir_real().unwrap();
    let mut optional = serde_json::Map::new();
    let mut versions = serde_json::Map::new();
    for (os, cpu) in [
        ("darwin", "x64"),
        ("darwin", "arm64"),
        ("linux", "x64"),
        ("linux", "arm64"),
        ("win32", "x64"),
        ("win32", "arm64"),
    ] {
        let alias = format!("@openai/codex-{os}-{cpu}");
        let platform_version = format!("0.162.0-{os}-{cpu}");
        optional.insert(
            alias,
            serde_json::Value::String(format!("npm:@openai/codex@{platform_version}")),
        );
        versions.insert(
            platform_version.clone(),
            serde_json::json!({
                "version": platform_version,
                "dependencies": null,
                "optionalDependencies": null,
                "os": [os],
                "cpu": [cpu],
                "dist": {
                    "tarball": format!("https://registry.example.test/codex-{os}-{cpu}.tgz"),
                    "integrity": "sha512-c2hhcmVk"
                }
            }),
        );
    }
    versions.insert(
        "0.162.0".to_string(),
        serde_json::json!({
            "version": "0.162.0",
            "dependencies": null,
            "optionalDependencies": optional,
            "os": null,
            "cpu": null,
            "dist": {
                "tarball": "https://registry.example.test/codex.tgz",
                "integrity": "sha512-c2hhcmVk"
            }
        }),
    );
    seed_shared_metadata(
        shared.path(),
        "@openai/codex",
        serde_json::json!({
            "name": "@openai/codex",
            "description": null,
            "versions": versions,
            "dist-tags": { "latest": "0.162.0" }
        }),
    );

    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
    let mut manifest = Manifest::new("codex-dlx", mgc_types::ecosystem::Ecosystem::Web);
    manifest.add_dep(
        DependencySpec::new(
            PackageName::new("@openai/codex").unwrap(),
            VersionRange::parse("0.162.0").unwrap(),
        ),
        false,
        false,
        false,
    );
    let graph = adapter.resolve(&manifest).await.unwrap();
    let root = graph
        .packages
        .iter()
        .find(|package| package.id.name_str() == "@openai/codex")
        .unwrap();
    let expected_platform = format!(
        "@openai/codex-{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    );

    assert!(
        graph
            .packages
            .iter()
            .any(|package| package.id.name_str() == expected_platform),
        "normalized graph dropped host platform package: {:?}",
        graph
            .packages
            .iter()
            .map(|package| package.id.name_str())
            .collect::<Vec<_>>()
    );
    assert!(
        root.deps
            .iter()
            .any(|dependency| dependency.name_str() == expected_platform),
        "root package does not point to its platform alias"
    );
}

