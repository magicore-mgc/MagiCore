//! Opens an existing regular SQLite file read-only and rejects schema migration.
//! Mở file SQLite thường đã có ở chế độ chỉ đọc và từ chối schema migration.

use std::fs;
use std::path::Path;

use rusqlite::{Connection, OpenFlags};

use super::super::schema::{STORE_DB_BUSY_TIMEOUT, STORE_DB_SCHEMA_VERSION};
use super::error::ReadOnlyOpenError;
use crate::database::database::{claim_writer_gate::shared_claim_write_gate, types::Database};

fn open_read_only(path: &Path) -> Result<Database, ReadOnlyOpenError> {
    let metadata = fs::symlink_metadata(path).map_err(|source| ReadOnlyOpenError::InspectPath {
        path: display_path(path),
        source,
    })?;
    if metadata.file_type().is_symlink() {
        return Err(ReadOnlyOpenError::SymlinkedDatabase {
            path: display_path(path),
        });
    }
    if !metadata.is_file() {
        return Err(ReadOnlyOpenError::NotRegularFile {
            path: display_path(path),
        });
    }

    let canonical_path =
        fs::canonicalize(path).map_err(|source| ReadOnlyOpenError::CanonicalizePath {
            path: display_path(path),
            source,
        })?;
    let confirmed =
        fs::symlink_metadata(path).map_err(|source| ReadOnlyOpenError::InspectPath {
            path: display_path(path),
            source,
        })?;
    if confirmed.file_type().is_symlink() {
        return Err(ReadOnlyOpenError::SymlinkedDatabase {
            path: display_path(path),
        });
    }
    if !confirmed.is_file() {
        return Err(ReadOnlyOpenError::NotRegularFile {
            path: display_path(path),
        });
    }

    let connection = Connection::open_with_flags(
        &canonical_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NOFOLLOW,
    )
    .map_err(|source| ReadOnlyOpenError::Open {
        path: display_path(&canonical_path),
        source,
    })?;
    connection
        .busy_timeout(STORE_DB_BUSY_TIMEOUT)
        .map_err(|source| ReadOnlyOpenError::BusyTimeout {
            path: display_path(&canonical_path),
            source,
        })?;

    let schema_version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|source| ReadOnlyOpenError::ReadSchemaVersion {
            path: display_path(&canonical_path),
            source,
        })?;
    if schema_version != STORE_DB_SCHEMA_VERSION {
        return Err(ReadOnlyOpenError::UnsupportedSchema {
            found: schema_version,
            supported: STORE_DB_SCHEMA_VERSION,
        });
    }

    let claim_write_gate = shared_claim_write_gate(&canonical_path).map_err(|source| {
        ReadOnlyOpenError::WriterGate {
            path: display_path(&canonical_path),
            source,
        }
    })?;
    Ok(Database {
        conn: connection,
        claim_write_gate,
    })
}

impl Database {
    /// Open an existing, supported store without creating a database or applying migrations.
    /// Mở store đã có và đúng schema, không tạo database hay áp migration.
    pub fn open_read_only(path: &Path) -> anyhow::Result<Self> {
        Ok(open_read_only(path)?)
    }
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
