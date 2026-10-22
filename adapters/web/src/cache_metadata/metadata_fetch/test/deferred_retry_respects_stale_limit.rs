//! Existing deferred-retry stale-age boundary test.
//! Test ranh giới tuổi stale khi retry bị trì hoãn hiện có.

use super::*;

#[tokio::test]
async fn test_retry_deferred_does_not_bypass_max_stale_limit() {
    let _env_guard = env_test_lock().lock().unwrap();
    let shared = tempdir_real().unwrap();
    let registry = native::npm_registry::NpmRegistry::new("http://127.0.0.1:9");
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
    unsafe { std::env::set_var("MAGICORE_WEB_METADATA_MAX_STALE_SECS", "60") };
    cache
        .write_metadata_record(
            "react",
            &metadata,
            Some("\"react-v1\"".to_string()),
            1,
            Some(current_unix_secs().saturating_add(60)),
            "http://127.0.0.1:9",
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
