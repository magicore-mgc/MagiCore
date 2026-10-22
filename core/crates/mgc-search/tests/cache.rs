//! Tests for search cache
//! Tests cho search cache

use mgc_search::cache::SearchCache;
use mgc_search::types::*;
use tempfile::tempdir;

#[test]
fn test_cache_insert_and_get() {
    let temp_dir = tempdir().expect("cache test fixture operation succeeds");
    let db_path = temp_dir.path().join("test_cache.db");

    let cache =
        SearchCache::new_with_path(&db_path).expect("cache test fixture operation succeeds");

    let results = vec![SearchResult {
        name: "test".to_string(),
        registry: Registry::Npm,
        full_path: "test".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package".to_string(),
        metadata: ResultMetadata::default(),
        score: 90.0,
    }];

    cache
        .insert("test", &results)
        .expect("cache test fixture operation succeeds");

    let cached = cache
        .get("test")
        .expect("cache test fixture operation succeeds");
    assert!(cached.is_some());
    assert_eq!(
        cached.expect("cache test fixture operation succeeds")[0].name,
        "test"
    );
}

#[test]
fn test_cache_miss() {
    let temp_dir = tempdir().expect("cache test fixture operation succeeds");
    let db_path = temp_dir.path().join("test_cache.db");

    let cache =
        SearchCache::new_with_path(&db_path).expect("cache test fixture operation succeeds");

    let cached = cache
        .get("nonexistent")
        .expect("cache test fixture operation succeeds");
    assert!(cached.is_none());
}

#[test]
fn test_track_choice() {
    let temp_dir = tempdir().expect("cache test fixture operation succeeds");
    let db_path = temp_dir.path().join("test_cache.db");

    let cache =
        SearchCache::new_with_path(&db_path).expect("cache test fixture operation succeeds");

    // Track 3 times
    cache
        .track_choice("gin", Registry::Go)
        .expect("cache test fixture operation succeeds");
    cache
        .track_choice("gin", Registry::Go)
        .expect("cache test fixture operation succeeds");
    cache
        .track_choice("gin", Registry::Go)
        .expect("cache test fixture operation succeeds");

    // Should return Go registry (installed 3+ times)
    let choice = cache
        .get_user_choice("gin")
        .expect("cache test fixture operation succeeds");
    assert_eq!(choice, Some(Registry::Go));
}

#[test]
fn test_track_choice_multiple_registries() {
    let temp_dir = tempdir().expect("cache test fixture operation succeeds");
    let db_path = temp_dir.path().join("test_cache.db");

    let cache =
        SearchCache::new_with_path(&db_path).expect("cache test fixture operation succeeds");

    // Track npm twice
    cache
        .track_choice("cors", Registry::Npm)
        .expect("cache test fixture operation succeeds");
    cache
        .track_choice("cors", Registry::Npm)
        .expect("cache test fixture operation succeeds");

    // Track go 4 times (more popular)
    cache
        .track_choice("cors", Registry::Go)
        .expect("cache test fixture operation succeeds");
    cache
        .track_choice("cors", Registry::Go)
        .expect("cache test fixture operation succeeds");
    cache
        .track_choice("cors", Registry::Go)
        .expect("cache test fixture operation succeeds");
    cache
        .track_choice("cors", Registry::Go)
        .expect("cache test fixture operation succeeds");

    // Should prefer Go (higher install count)
    let choice = cache
        .get_user_choice("cors")
        .expect("cache test fixture operation succeeds");
    assert_eq!(choice, Some(Registry::Go));
}

#[test]
fn test_user_choice_below_threshold() {
    let temp_dir = tempdir().expect("cache test fixture operation succeeds");
    let db_path = temp_dir.path().join("test_cache.db");

    let cache =
        SearchCache::new_with_path(&db_path).expect("cache test fixture operation succeeds");

    // Track only 2 times (below threshold of 3)
    cache
        .track_choice("axios", Registry::Npm)
        .expect("cache test fixture operation succeeds");
    cache
        .track_choice("axios", Registry::Npm)
        .expect("cache test fixture operation succeeds");

    // Should not return choice (below threshold)
    let choice = cache
        .get_user_choice("axios")
        .expect("cache test fixture operation succeeds");
    assert!(choice.is_none());
}

#[test]
fn test_cache_update() {
    let temp_dir = tempdir().expect("cache test fixture operation succeeds");
    let db_path = temp_dir.path().join("test_cache.db");

    let cache =
        SearchCache::new_with_path(&db_path).expect("cache test fixture operation succeeds");

    let results1 = vec![SearchResult {
        name: "test".to_string(),
        registry: Registry::Npm,
        full_path: "test".to_string(),
        version: "1.0.0".to_string(),
        description: "Old version".to_string(),
        metadata: ResultMetadata::default(),
        score: 80.0,
    }];

    cache
        .insert("test", &results1)
        .expect("cache test fixture operation succeeds");

    let results2 = vec![SearchResult {
        name: "test".to_string(),
        registry: Registry::Npm,
        full_path: "test".to_string(),
        version: "2.0.0".to_string(),
        description: "New version".to_string(),
        metadata: ResultMetadata::default(),
        score: 90.0,
    }];

    cache
        .insert("test", &results2)
        .expect("cache test fixture operation succeeds");

    let cached = cache
        .get("test")
        .expect("cache test fixture operation succeeds")
        .expect("cache test fixture operation succeeds");
    assert_eq!(cached[0].version, "2.0.0");
    assert_eq!(cached[0].description, "New version");
}
