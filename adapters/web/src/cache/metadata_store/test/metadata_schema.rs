//! Existing shared metadata schema-compatibility test.
//! Test tương thích schema metadata dùng chung hiện có.

use crate::cache::SharedWebCache;

fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}

fn reg_key(url: &str) -> String {
    url.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

#[test]
fn test_shared_metadata_cache_ignores_pre_optional_peer_schema_records() {
    let shared = tempdir_real().unwrap();
    let cache = SharedWebCache {
        root: shared.path().to_path_buf(),
    };
    let registry_url = "http://127.0.0.1:9";
    let legacy_path = shared
        .path()
        .join("metadata")
        .join(reg_key(registry_url))
        .join("react")
        .join("metadata.json");
    std::fs::create_dir_all(legacy_path.parent().unwrap()).unwrap();
    std::fs::write(
        &legacy_path,
        r#"{"fetched_at":1,"etag":"legacy","metadata":{"name":"react","description":null,"versions":{"18.2.0":{"version":"18.2.0","peerDependencies":{"optional-peer":"*"}}},"dist-tags":{"latest":"18.2.0"}}}"#,
    )
    .unwrap();

    assert!(
        cache
            .read_metadata("react", registry_url)
            .unwrap()
            .is_none(),
        "legacy metadata lacks peerDependenciesMeta and must be a cache miss"
    );
}
