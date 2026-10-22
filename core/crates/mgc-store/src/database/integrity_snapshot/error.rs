//! Typed failures from the CAS integrity-count query.
//! Lỗi có kiểu của truy vấn đếm toàn vẹn CAS.

/// Failure while reading CAS integrity counters.
/// Lỗi khi đọc bộ đếm toàn vẹn CAS.
#[derive(Debug, thiserror::Error)]
pub enum CasIntegritySnapshotError {
    /// SQLite rejected or failed the single integrity snapshot query.
    /// SQLite từ chối hoặc không thực hiện được truy vấn snapshot toàn vẹn.
    #[error("CAS integrity count query failed: {0}")]
    Query(#[source] rusqlite::Error),
}
