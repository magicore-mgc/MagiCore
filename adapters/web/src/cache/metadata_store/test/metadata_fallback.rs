//! Shared metadata cache fallback behavior test.
//! Test fallback hành vi cache metadata dùng chung.

use super::support::{seed_shared_metadata, tempdir_real};
use crate::WebAdapter;
use mgc_types::PackageName;
use mgc_types::adapter::{AddOptions, PackageAdapter};

#[tokio::test]
async fn test_add_uses_shared_metadata_cache_when_registry_is_unavailable() {
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

    assert_eq!(package_id.version().to_string(), "18.2.0");
    let package_json = std::fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(!package_json.contains("\"react\": \"^18.2.0\""));
}
