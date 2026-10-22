/// Convert a non-negative Rust integer to SQLite's signed INTEGER range.
/// Chuyển số không âm của Rust vào miền INTEGER có dấu của SQLite.
pub(crate) fn sqlite_i64(value: u64) -> rusqlite::Result<i64> {
    i64::try_from(value).map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))
}

/// Decode SQLite INTEGER without wrapping negative or out-of-range values.
/// Đọc INTEGER SQLite và từ chối giá trị âm hoặc vượt miền thay vì wrap.
pub(crate) fn u64_from_sqlite(value: i64, column: usize) -> rusqlite::Result<u64> {
    u64::try_from(value).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            column,
            rusqlite::types::Type::Integer,
            Box::new(error),
        )
    })
}
