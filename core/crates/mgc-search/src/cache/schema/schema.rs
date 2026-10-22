//! Create the search cache tables and indexes.
//! Tạo các bảng và index của cache tìm kiếm.

use rusqlite::Connection;

use super::super::error::SearchCacheError;

/// Initialize the existing cache schema without changing its SQL contract.
/// Khởi tạo schema hiện tại mà không đổi hợp đồng SQL.
pub(in crate::cache) fn initialize(db: &Connection) -> std::result::Result<(), SearchCacheError> {
    db.execute(
        "CREATE TABLE IF NOT EXISTS search_cache (
            query TEXT PRIMARY KEY,
            results TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            ttl INTEGER DEFAULT 604800
        )",
        [],
    )?;

    db.execute(
        "CREATE INDEX IF NOT EXISTS idx_search_cache_timestamp
         ON search_cache(timestamp)",
        [],
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS user_choices (
            package_name TEXT,
            registry TEXT,
            install_count INTEGER DEFAULT 1,
            last_used INTEGER NOT NULL,
            PRIMARY KEY (package_name, registry)
        )",
        [],
    )?;

    db.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_choices_count
         ON user_choices(install_count DESC)",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
#[path = "test/schema.rs"]
mod tests;
