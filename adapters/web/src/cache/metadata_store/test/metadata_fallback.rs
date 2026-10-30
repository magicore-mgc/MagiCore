//! Shared metadata cache fallback behavior test.
//! Test fallback hành vi cache metadata dùng chung.

#![allow(unsafe_code)]

use super::support::{seed_shared_metadata, tempdir_real};
use crate::WebAdapter;
use crate::environment_test_guard;
use mgc_types::PackageName;
use mgc_types::adapter::{AddOptions, PackageAdapter};

#[tokio::test]
async fn test_add_uses_shared_metadata_cache_when_registry_is_unavailable() {
    // The shared cache stands in for a registry that has already been resolved
    // before, and the default 24h quarantine would otherwise refuse the fixture
    // because it carries no publish timestamps. This test's subject is the
    // offline cache fallback, not the age gate, so it states the operator opt-out
    // the way a user with a timestamp-less private registry would.
    // (Cache dùng chung thay cho registry đã resolve trước; quarantine mặc định
    // 24h sẽ từ chối fixture vì thiếu timestamp. Chủ đề của test là fallback
    // cache offline, nên nó nêu rõ opt-out giống user dùng registry nội bộ
    // không có timestamp.)
    let _env = environment_test_guard();
    // SAFETY: test-only env mutation under the process env lock.
    // AN TOÀN: chỉ test đổi env dưới lock env của process.
    unsafe { std::env::set_var("MAGICORE_ALLOW_UNTRUSTED", "1") };
    let shared = tempdir_real().unwrap();

    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        serde_json::json!({
            "name": "demo",
            "version": "0.1.0"
        })
        .to_string(),
    )
    .unwrap();

    seed_shared_metadata(
        shared.path(),
        "react",
        serde_json::json!({
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
                        "tarball": "http://127.0.0.1:9/react.tgz",
                        "integrity": null
                    }
                }
            },
            "dist-tags": {
                "latest": "18.2.0"
            }
        }),
    );

    let adapter = WebAdapter::with_registry_and_shared_cache(
        "http://127.0.0.1:9".into(),
        shared.path().to_path_buf(),
    );
    let prepared = adapter
        .prepare_add(
            dir.path(),
            &PackageName::new("react").unwrap(),
            None,
            AddOptions::default(),
        )
        .await
        .unwrap();
    let package_id = prepared.id;

    // SAFETY: test-only env mutation under the process env lock.
    // AN TOÀN: chỉ test đổi env dưới lock env của process.
    unsafe { std::env::remove_var("MAGICORE_ALLOW_UNTRUSTED") };

    assert_eq!(package_id.version().to_string(), "18.2.0");
    let package_json = std::fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(!package_json.contains("\"react\": \"^18.2.0\""));
}
