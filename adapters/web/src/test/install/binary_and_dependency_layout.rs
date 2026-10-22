//! Web binary-link and dependency-layout integration tests.
//! Test tích hợp binary link và dependency layout Web.

use super::*;

#[tokio::test]
async fn test_install_materializes_node_modules_bin_links() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "vite": "^8.1.4"
            }
        })
        .to_string(),
    )
    .unwrap();

    let package_id = PackageId::new(
        PackageName::new("vite").unwrap(),
        Version::parse("8.1.4").unwrap(),
    );
    let integrity = seed_cached_tarball_with_files(
        dir.path(),
        &package_id,
        &[
            (
                "package/package.json",
                br#"{"name":"vite","version":"8.1.4","bin":"bin/vite.js"}"#.as_slice(),
            ),
            (
                "package/bin/vite.js",
                b"#!/usr/bin/env node\nconsole.log('vite')\n",
            ),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id,
            integrity,
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: true,
        }],
    };

    let adapter = WebAdapter::new().unwrap();
    adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();

    assert!(dir.path().join("node_modules/.bin").exists());
    assert!(dir.path().join("node_modules/.bin/vite").exists());
}

#[tokio::test]
async fn test_install_materializes_nested_conflicting_dependency_versions() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "@nuxt/kit": "1.0.0",
                "legacy-tool": "1.0.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let nuxt_kit = PackageId::new(
        PackageName::new("@nuxt/kit").unwrap(),
        Version::parse("1.0.0").unwrap(),
    );
    let legacy_tool = PackageId::new(
        PackageName::new("legacy-tool").unwrap(),
        Version::parse("1.0.0").unwrap(),
    );
    let semver7 = PackageId::new(
        PackageName::new("semver").unwrap(),
        Version::parse("7.8.5").unwrap(),
    );
    let semver6 = PackageId::new(
        PackageName::new("semver").unwrap(),
        Version::parse("6.3.1").unwrap(),
    );

    let nuxt_kit_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &nuxt_kit,
        &[(
            "package/package.json",
            br#"{"name":"@nuxt/kit","version":"1.0.0"}"#.as_slice(),
        )],
    );
    let legacy_tool_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &legacy_tool,
        &[(
            "package/package.json",
            br#"{"name":"legacy-tool","version":"1.0.0"}"#.as_slice(),
        )],
    );
    let semver7_integrity = seed_cached_tarball_with_files(
            dir.path(),
            &semver7,
            &[
                (
                    "package/package.json",
                    br#"{"name":"semver","version":"7.8.5","exports":{"./functions/satisfies.js":"./functions/satisfies.js"}}"#
                        .as_slice(),
                ),
                ("package/functions/satisfies.js", b"export default true;\n"),
            ],
        );
    let semver6_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &semver6,
        &[(
            "package/package.json",
            br#"{"name":"semver","version":"6.3.1"}"#.as_slice(),
        )],
    );

    let graph = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: nuxt_kit.clone(),
                integrity: nuxt_kit_integrity,
                tarball_url: String::new(),
                deps: vec![semver7.clone()],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: legacy_tool.clone(),
                integrity: legacy_tool_integrity,
                tarball_url: String::new(),
                deps: vec![semver6.clone()],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: semver7.clone(),
                integrity: semver7_integrity,
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
            ResolvedPackage {
                id: semver6.clone(),
                integrity: semver6_integrity,
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: false,
                dev: false,
            },
        ],
    };

    let adapter = WebAdapter::new().unwrap();
    adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();

    let nested_nuxt_semver = dir
        .path()
        .join("node_modules")
        .join("@nuxt")
        .join("kit")
        .join("node_modules")
        .join("semver");
    assert!(!dir.path().join("node_modules").join("semver").exists());
    assert!(nested_nuxt_semver.exists());
    assert!(
        nested_nuxt_semver
            .join("functions")
            .join("satisfies.js")
            .exists()
    );
    assert_eq!(
        installed_package_version(&nested_nuxt_semver)
            .unwrap()
            .to_string(),
        "7.8.5"
    );
    assert_eq!(
        installed_package_version(
            &dir.path()
                .join("node_modules")
                .join("legacy-tool")
                .join("node_modules")
                .join("semver"),
        )
        .unwrap()
        .to_string(),
        "6.3.1"
    );
}

