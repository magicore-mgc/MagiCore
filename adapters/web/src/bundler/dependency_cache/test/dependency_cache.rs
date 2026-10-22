//! Session-cache and fail-closed API tests.
//! Test cache theo session và API fail-closed.

use std::path::PathBuf;

use super::DepsCache;
use crate::bundler::dependency_cache::CachedDep;

#[tokio::test]
async fn rejects_invalid_package_before_cache_or_path_lookup() {
    let cache = DepsCache::new(PathBuf::from("missing-node-modules"));

    assert!(cache.get_or_bundle("../outside").await.is_none());
    assert!(cache.get_or_bundle("@scope//package").await.is_none());
}

#[tokio::test]
async fn returns_a_cloned_session_cache_hit_without_resolving_files() {
    let cache = DepsCache::new(PathBuf::from("missing-node-modules"));
    let cached = CachedDep {
        js: "export default 7;".to_string(),
        css: Some(".value { color: red }".to_string()),
    };
    cache
        .inner
        .write()
        .await
        .insert("sample".to_string(), cached.clone());

    let returned = cache.get_or_bundle("sample").await.unwrap();

    assert_eq!(returned.js, cached.js);
    assert_eq!(returned.css, cached.css);
}
