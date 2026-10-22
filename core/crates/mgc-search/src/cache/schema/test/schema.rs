//! Tests for SQLite cache schema initialization.
//! Test khởi tạo schema SQLite của cache.

use rusqlite::Connection;

use super::initialize;

#[test]
fn schema_initialization_is_idempotent_and_creates_objects() {
    let db = Connection::open_in_memory().expect("schema query fixture succeeds");

    initialize(&db).expect("schema query fixture succeeds");
    initialize(&db).expect("schema query fixture succeeds");

    let object_count: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE name IN (
                 'search_cache',
                 'idx_search_cache_timestamp',
                 'user_choices',
                 'idx_user_choices_count'
             )",
            [],
            |row| row.get(0),
        )
        .expect("schema query fixture succeeds");

    assert_eq!(object_count, 4);
}
