//! SQLite operations for serialized search results.
//! Thao tác SQLite cho kết quả tìm kiếm đã serialize.

use rusqlite::{Connection, params};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::SearchResult;

use super::super::error::SearchCacheError;

/// Read one unexpired cached result set.
/// Đọc một bộ kết quả cache còn hạn.
pub(in crate::cache) fn get(
    db: &Connection,
    query: &str,
) -> std::result::Result<Option<Vec<SearchResult>>, SearchCacheError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let mut stmt =
        db.prepare("SELECT results, timestamp, ttl FROM search_cache WHERE query = ?")?;

    let result = stmt.query_row([query], |row| {
        let results_json: String = row.get(0)?;
        let timestamp: i64 = row.get(1)?;
        let ttl: i64 = row.get(2)?;

        if now - timestamp > ttl {
            return Ok(None);
        }

        let results: Vec<SearchResult> = serde_json::from_str(&results_json)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;

        Ok(Some(results))
    });

    match result {
        Ok(results) => Ok(results),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(SearchCacheError::Sqlite(error)),
    }
}

/// Replace the cached result set for one query.
/// Thay thế bộ kết quả cache của một truy vấn.
pub(in crate::cache) fn insert(
    db: &Connection,
    query: &str,
    results: &[SearchResult],
) -> std::result::Result<(), SearchCacheError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let results_json = serde_json::to_string(results)?;

    db.execute(
        "INSERT OR REPLACE INTO search_cache (query, results, timestamp, ttl)
         VALUES (?1, ?2, ?3, 604800)",
        params![query, results_json, now],
    )?;

    Ok(())
}

#[cfg(test)]
#[path = "test/result_cache.rs"]
mod tests;
