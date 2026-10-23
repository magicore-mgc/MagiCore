//! Web package materialization integration tests.
//! Test tích hợp materialize package Web.

use super::*;

#[tokio::test]
async fn test_install_materializes_manifest_dependency_into_node_modules() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "private": true,
            "type": "module",
            "dependencies": { "tailwindcss": "^3.4.0" },
            "scripts": {
                "dev": "mgc web dev"
            }
        })
        .to_string(),
    )
    .unwrap();

    let adapter = WebAdapter::new().unwrap();
    let name = PackageName::new("tailwindcss").unwrap();
    let manifest = adapter.parse_manifest(dir.path()).await.unwrap();
    assert!(manifest.find_dep("tailwindcss").is_some());
    let package_json = std::fs::read_to_string(dir.path().join("package.json")).unwrap();
    let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();
    assert_eq!(package_json["private"], true);
    assert_eq!(package_json["type"], "module");
    assert_eq!(package_json["scripts"]["dev"], "mgc web dev");

    let package_id = PackageId::new(name, Version::parse("3.4.0").unwrap());
    let integrity = seed_cached_tarball(dir.path(), &package_id);
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id.clone(),
            integrity,
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };
    let summary = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.added, vec![package_id]);
    assert!(
        dir.path()
            .join("node_modules")
            .join("tailwindcss")
            .join("package.json")
            .exists()
    );
    assert!(
        dir.path()
            .join("node_modules")
            .join("tailwindcss")
            .join("index.css")
            .exists()
    );

    let lock = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    let parsed: Lockfile = mgc_lockfile::parse_lockfile(&lock).unwrap();
    // Deliberate v3 bump (Phase 1): the web writer now emits schema v3.
    // Nâng lên v3 có chủ đích (Phase 1): web writer giờ ghi schema v3.
    assert_eq!(parsed.version, "3");
    assert_eq!(parsed.packages.len(), 1);
    assert_eq!(parsed.packages[0].name, "tailwindcss");
    assert_eq!(parsed.packages[0].version, "3.4.0");
}

#[tokio::test]
async fn test_install_multiple_packages_from_cache() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0",
                "tailwindcss": "^4.3.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let tailwind = PackageId::new(
        PackageName::new("tailwindcss").unwrap(),
        Version::parse("4.3.2").unwrap(),
    );

    let react_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export const version = '18.2.0';"),
        ],
    );
    let tailwind_integrity = seed_cached_tarball_with_files(
        dir.path(),
        &tailwind,
        &[
            (
                "package/package.json",
                br#"{"name":"tailwindcss","version":"4.3.2"}"#.as_slice(),
            ),
            ("package/index.css", b"@import 'tailwindcss';"),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![
            ResolvedPackage {
                id: react.clone(),
                integrity: react_integrity,
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
            ResolvedPackage {
                id: tailwind.clone(),
                integrity: tailwind_integrity,
                tarball_url: String::new(),
                deps: vec![],
                peer_deps: vec![],
                direct: true,
                dev: false,
            },
        ],
    };

    let adapter = WebAdapter::new().unwrap();
    let summary = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.added.len(), 2);
    assert!(summary.bytes_from_cache > 0);
    assert!(dir.path().join("node_modules/react/index.js").exists());
    assert!(
        dir.path()
            .join("node_modules/tailwindcss/index.css")
            .exists()
    );

    let installed = adapter.list(dir.path()).await.unwrap();
    assert_eq!(installed.len(), 2);
}

#[tokio::test]
async fn test_install_skips_when_matching_package_is_already_materialized() {
    // Same loopback opt-in as the cache test above (default deny).
    let _env_guard = env_test_lock().lock().unwrap();
    let old_insecure = std::env::var_os("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST");
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1") };
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "zod": "^4.4.3"
            }
        })
        .to_string(),
    )
    .unwrap();

    let package_id = PackageId::new(
        PackageName::new("zod").unwrap(),
        Version::parse("4.4.3").unwrap(),
    );
    let pkg_dir = dir.path().join("node_modules").join("zod");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(
        pkg_dir.join("package.json"),
        br#"{"name":"zod","version":"4.4.3"}"#,
    )
    .unwrap();
    std::fs::write(pkg_dir.join("marker.txt"), "keep-me").unwrap();

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id.clone(),
            integrity: String::new(),
            tarball_url: "http://127.0.0.1:9/unreachable.tgz".into(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    let adapter = WebAdapter::with_registry("http://127.0.0.1:9".into()).unwrap();
    let summary = adapter
        .install(
            &graph,
            dir.path(),
            InstallOptions {
                legacy_flat: true,
                ..InstallOptions::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(summary.added, vec![package_id]);
    assert_eq!(
        std::fs::read_to_string(pkg_dir.join("marker.txt")).unwrap(),
        "keep-me"
    );

    let lock = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    let parsed: Lockfile = mgc_lockfile::parse_lockfile(&lock).unwrap();
    // Deliberate v3 bump (Phase 1): the web writer now emits schema v3.
    // Nâng lên v3 có chủ đích (Phase 1): web writer giờ ghi schema v3.
    assert_eq!(parsed.version, "3");
    assert_eq!(parsed.packages[0].version, "4.4.3");
    restore_env_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", old_insecure);
}

#[tokio::test]
async fn test_install_materializes_scoped_package() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "devDependencies": {
                "@types/node": "26.1.1"
            }
        })
        .to_string(),
    )
    .unwrap();

    let package_id = PackageId::new(
        PackageName::new("@types/node").unwrap(),
        Version::parse("26.1.1").unwrap(),
    );
    let integrity = seed_cached_tarball_with_files(
        dir.path(),
        &package_id,
        &[
            (
                "package/package.json",
                br#"{"name":"@types/node","version":"26.1.1"}"#.as_slice(),
            ),
            ("package/index.d.ts", b"export {};"),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id.clone(),
            integrity,
            tarball_url: String::new(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: true,
        }],
    };

    let adapter = WebAdapter::new().unwrap();
    let summary = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.added, vec![package_id]);
    assert!(
        dir.path()
            .join("node_modules")
            .join("@types")
            .join("node")
            .join("package.json")
            .exists()
    );
    assert!(
        dir.path()
            .join("node_modules")
            .join("@types")
            .join("node")
            .join("index.d.ts")
            .exists()
    );
}
