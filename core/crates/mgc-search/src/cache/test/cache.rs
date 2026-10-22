//! Tests for the SearchCache facade.
//! Test cho facade SearchCache.

use super::SearchCache;
use tempfile::tempdir;

#[test]
fn constructor_creates_parent_and_schema() {
    let directory = tempdir().expect("cache constructor fixture succeeds");
    let path = directory.path().join("nested/search-cache.sqlite");

    let cache = SearchCache::new_with_path(&path).expect("cache constructor fixture succeeds");

    assert!(path.is_file());
    assert!(
        cache
            .get("missing")
            .expect("cache constructor fixture succeeds")
            .is_none()
    );
}
