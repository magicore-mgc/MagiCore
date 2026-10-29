//! Existing maximum-stale-age rejection behavior test.
//! Test hành vi từ chối metadata vượt tuổi stale tối đa hiện có.

use super::*;

#[tokio::test]
async fn test_stale_metadata_too_old_is_not_reused_when_network_fails() {
    let _env_guard = crate::environment_test_guard();
    let shared = tempdir_real().unwrap();
    let Some(listener) = bind_test_listener().await else {
        return;
    };
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
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

    let previous = std::env::var_os("MAGICORE_WEB_METADATA_MAX_STALE_SECS");
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("MAGICORE_WEB_METADATA_MAX_STALE_SECS", "60") };
    cache
        .write_metadata_record(
            "react",
            &metadata,
            Some("\"react-v1\"".to_string()),
            1,
            None,
            &format!("http://{addr}"),
        )
        .unwrap();

    let err = load_metadata_by_name_with_fallback("react", &registry, Some(&cache))
        .await
        .unwrap_err();
    restore_env_var("MAGICORE_WEB_METADATA_MAX_STALE_SECS", previous);

    assert!(
        err.to_string()
            .contains("cached metadata is too old to reuse")
    );
}
