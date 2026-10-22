//! SQLite operations for learned package registry choices.
//! Thao tác SQLite cho lựa chọn registry package đã học.

use rusqlite::{Connection, params};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::Registry;

use super::super::error::SearchCacheError;

/// Increment the install count for a package and registry pair.
/// Tăng số lần cài cho cặp package và registry.
pub(in crate::cache) fn track_choice(
    db: &Connection,
    package_name: &str,
    registry: Registry,
) -> std::result::Result<(), SearchCacheError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let registry_str = registry.as_str();

    let updated = db.execute(
        "UPDATE user_choices
         SET install_count = install_count + 1, last_used = ?3
         WHERE package_name = ?1 AND registry = ?2",
        params![package_name, registry_str, now],
    )?;

    if updated == 0 {
        db.execute(
            "INSERT INTO user_choices (package_name, registry, install_count, last_used)
             VALUES (?1, ?2, 1, ?3)",
            params![package_name, registry_str, now],
        )?;
    }

    Ok(())
}

/// Return the most-used registry after the current three-choice threshold.
/// Trả registry được dùng nhiều nhất sau ngưỡng ba lựa chọn hiện hành.
pub(in crate::cache) fn get_user_choice(
    db: &Connection,
    package_name: &str,
) -> std::result::Result<Option<Registry>, SearchCacheError> {
    let mut stmt = db.prepare(
        "SELECT registry FROM user_choices
         WHERE package_name = ? AND install_count >= 3
         ORDER BY install_count DESC, last_used DESC
         LIMIT 1",
    )?;

    let result = stmt.query_row([package_name], |row| {
        let registry_str: String = row.get(0)?;
        Ok(Registry::parse(&registry_str))
    });

    match result {
        Ok(Some(registry)) => Ok(Some(registry)),
        Ok(None) => Ok(None),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(SearchCacheError::Sqlite(error)),
    }
}

#[cfg(test)]
#[path = "test/user_choice.rs"]
mod tests;
