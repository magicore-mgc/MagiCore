//! Existing stale-metadata retry-cooldown behavior test.
//! Test hành vi cooldown retry metadata stale hiện có.

use super::*;

#[tokio::test]
async fn test_stale_metadata_failure_sets_retry_cooldown() {
    let _env_guard = crate::environment_test_guard();
    let shared = tempdir_real().unwrap();
    let Some(listener) = bind_test_listener().await else {
        return;
    };
    let addr = listener.local_addr().unwrap();
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_for_server = hits.clone();

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            hits_for_server.fetch_add(1, Ordering::SeqCst);
            let mut buf = [0u8; 2048];
            let _ = stream.read(&mut buf).await;
            let _ = stream
                .write_all(b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 3\r\n\r\nbad")
                .await;
        }
    });

    let registry = native::npm_registry::NpmRegistry::new(&format!("http://{addr}"));
    let cache = SharedWebCache {
        root: shared.path().to_path_buf(),
    };
    let metadata: native::npm_registry::PackageMetadata =
        serde_json::from_value(serde_json::json!({
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
                        "tarball": "http://example.test/react.tgz",
                        "integrity": "sha512-react"
                    }
                }
            },
            "dist-tags": {
                "latest": "18.2.0"
            }
        }))
        .unwrap();

    cache
        .write_metadata_record(
            "react",
            &metadata,
            Some("\"react-v1\"".to_string()),
            current_unix_secs().saturating_sub(metadata_ttl_secs() + 1),
            None,
            &format!("http://{addr}"),
        )
        .unwrap();

    let previous_max_stale = std::env::var_os("MAGICORE_WEB_METADATA_MAX_STALE_SECS");
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("MAGICORE_WEB_METADATA_MAX_STALE_SECS", "604800") };

    let first = load_metadata_by_name_with_fallback("react", &registry, Some(&cache))
        .await
        .unwrap();
    let hits_after_first = hits.load(Ordering::SeqCst);
    let second = load_metadata_by_name_with_fallback("react", &registry, Some(&cache))
        .await
        .unwrap();
    restore_env_var("MAGICORE_WEB_METADATA_MAX_STALE_SECS", previous_max_stale);

    assert_eq!(first.name, "react");
    assert_eq!(second.name, "react");
    assert!(hits_after_first >= 1);
    assert_eq!(hits.load(Ordering::SeqCst), hits_after_first);

    let cached = cache
        .read_metadata("react", &format!("http://{addr}"))
        .unwrap()
        .unwrap();
    assert!(cached.stale_retry_after.is_some());
    assert!(metadata_record_retry_deferred(&cached));
}
