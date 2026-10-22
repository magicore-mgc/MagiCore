//! Tests for search result SQLite operations.
//! Test thao tác SQLite của kết quả tìm kiếm.

use rusqlite::Connection;

use crate::types::{Registry, ResultMetadata, SearchResult};

use super::{get, insert};

fn sample_result(version: &str, description: &str) -> SearchResult {
    SearchResult {
        name: "sample".to_string(),
        registry: Registry::Npm,
        full_path: "sample".to_string(),
        version: version.to_string(),
        description: description.to_string(),
        metadata: ResultMetadata::default(),
        score: 90.0,
    }
}

#[test]
fn inserted_results_round_trip_and_replace() {
    let db = Connection::open_in_memory().expect("result-cache SQL operation succeeds");
    super::super::super::schema::initialize(&db).expect("result-cache SQL operation succeeds");

    insert(&db, "sample", &[sample_result("1.0.0", "old")])
        .expect("result-cache SQL operation succeeds");
    insert(&db, "sample", &[sample_result("2.0.0", "new")])
        .expect("result-cache SQL operation succeeds");

    let cached = get(&db, "sample")
        .expect("result-cache SQL operation succeeds")
        .expect("result-cache SQL operation succeeds");

    assert_eq!(cached[0].version, "2.0.0");
    assert_eq!(cached[0].description, "new");
}

#[test]
fn missing_query_returns_no_results() {
    let db = Connection::open_in_memory().expect("result-cache SQL operation succeeds");
    super::super::super::schema::initialize(&db).expect("result-cache SQL operation succeeds");

    assert!(
        get(&db, "missing")
            .expect("result-cache SQL operation succeeds")
            .is_none()
    );
}

#[test]
fn expired_result_is_a_cache_miss() {
    let db = Connection::open_in_memory().expect("result-cache SQL operation succeeds");
    super::super::super::schema::initialize(&db).expect("result-cache SQL operation succeeds");
    db.execute(
        "INSERT INTO search_cache (query, results, timestamp, ttl)
         VALUES ('expired', '[]', 1, 0)",
        [],
    )
    .expect("result-cache SQL operation succeeds");

    assert!(
        get(&db, "expired")
            .expect("result-cache SQL operation succeeds")
            .is_none()
    );
}
