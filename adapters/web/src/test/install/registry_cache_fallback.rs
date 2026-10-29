//! Web registry and cache fallback integration tests.
//! Test tích hợp registry và cache fallback Web.

use super::*;

#[tokio::test]
async fn test_install_uses_cache_when_registry_is_unavailable() {
    // Loopback http tarballs require explicit opt-in (default deny) —
    // hold the env lock so parallel tests never observe a torn value.
    let _env_guard = crate::environment_test_guard();
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
                "tailwindcss": "^4.3.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let package_id = PackageId::new(
        PackageName::new("tailwindcss").unwrap(),
        Version::parse("4.3.2").unwrap(),
    );
    let integrity = seed_cached_tarball(dir.path(), &package_id);

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id.clone(),
            integrity,
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
    assert!(summary.bytes_from_cache > 0);
    restore_env_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", old_insecure);
}

#[tokio::test]
async fn test_install_uses_shared_tarball_cache_for_new_project() {
    let shared = tempdir_real().unwrap();

    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let integrity = seed_shared_tarball_with_files(
        shared.path(),
        &react,
        &[
            (
                "package/package.json",
                br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
            ),
            ("package/index.js", b"export default 'react';"),
        ],
    );

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: react.clone(),
            integrity,
            tarball_url: "http://127.0.0.1:9/unreachable.tgz".into(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
    let summary = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.added, vec![react.clone()]);
    assert!(summary.bytes_from_cache > 0);
    assert!(dir.path().join("node_modules/react/index.js").exists());
    // SEC-01: shared entries live under a registry-origin scope directory, so
    // assert through the scoped cache handle instead of the legacy layout.
    // SEC-01: entry dùng chung nằm dưới thư mục scope origin registry, nên
    // assert qua handle cache đã scope thay vì layout cũ.
    assert!(
        PackageCache::for_registry(shared.path().join("cache"), TEST_REGISTRY_URL)
            .unwrap()
            .contains_tarball(&react)
    );
}

#[tokio::test]
async fn test_install_recovers_from_corrupted_local_cache_using_shared_cache() {
    let shared = tempdir_real().unwrap();

    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let files = vec![
        (
            "package/package.json",
            br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
        ),
        ("package/index.js", b"export default 'react';".as_slice()),
    ];
    let good_tarball = build_tarball_bytes(&files);
    seed_shared_tarball_with_files(shared.path(), &react, &files);
    let local_layout = Layout::new(dir.path().join(".magicore").join("cache").join("web"));
    std::fs::create_dir_all(local_layout.root()).unwrap();
    let local_cache = PackageCache::for_registry(local_layout.cache_dir(), TEST_REGISTRY_URL).unwrap();
    local_cache.cache_tarball(&react, b"corrupted").unwrap();

    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: react.clone(),
            integrity: sri_sha512(&good_tarball),
            tarball_url: "http://127.0.0.1:9/unreachable.tgz".into(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
    let summary = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.added, vec![react.clone()]);
    assert!(dir.path().join("node_modules/react/index.js").exists());

    assert!(local_cache.get_tarball(&react).unwrap().is_none());
    let shared_cache = PackageCache::for_registry(shared.path().join("cache"), TEST_REGISTRY_URL).unwrap();
    let repaired = shared_cache.get_tarball(&react).unwrap().unwrap();
    assert_eq!(repaired, good_tarball);
}

#[tokio::test]
async fn test_install_fails_when_registry_is_unavailable_and_cache_is_missing() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "tailwindcss": "^4.3.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let package_id = PackageId::new(
        PackageName::new("tailwindcss").unwrap(),
        Version::parse("4.3.2").unwrap(),
    );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: package_id,
            integrity: String::new(),
            tarball_url: "http://127.0.0.1:9/unreachable.tgz".into(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    let adapter = WebAdapter::build("http://127.0.0.1:9".into(), None, vec![], None);
    let err = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap_err();
    assert!(err.to_string().contains("download failed"));
}

#[tokio::test]
async fn test_install_retries_flaky_tarball_download() {
    let _env_guard = crate::environment_test_guard();
    let _allow_insecure = ScopedEnvVar::set("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1");
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0",
            "dependencies": {
                "react": "^18.2.0"
            }
        })
        .to_string(),
    )
    .unwrap();

    let tarball = build_tarball_bytes(&[
        (
            "package/package.json",
            br#"{"name":"react","version":"18.2.0"}"#.as_slice(),
        ),
        ("package/index.js", b"export default 'react';"),
    ]);
    let integrity = sri_sha512(&tarball);
    let tarball_for_server = tarball.clone();
    let Some(listener) = bind_test_listener().await else {
        return;
    };
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let mut attempts = 0usize;
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            attempts += 1;
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf).await;
            if attempts == 1 {
                let _ = stream
                    .write_all(b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 4\r\n\r\nnope")
                    .await;
            } else {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                    tarball_for_server.len()
                );
                let _ = stream.write_all(response.as_bytes()).await;
                let _ = stream.write_all(&tarball_for_server).await;
            }
        }
    });

    let react = PackageId::new(
        PackageName::new("react").unwrap(),
        Version::parse("18.2.0").unwrap(),
    );
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: react.clone(),
            integrity,
            tarball_url: format!("http://{addr}/react-18.2.0.tgz"),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    let adapter = WebAdapter::build(TEST_REGISTRY_URL.into(), None, vec![], None);
    let summary = adapter
        .install(&graph, dir.path(), InstallOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.added, vec![react]);
    assert!(dir.path().join("node_modules/react/index.js").exists());
}
