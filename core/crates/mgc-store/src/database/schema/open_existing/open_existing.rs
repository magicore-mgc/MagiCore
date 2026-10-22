//! Opens an existing database for writes without CREATE and rejects final symlinks.
//! Mở database có sẵn để ghi, không CREATE và từ chối symlink ở component cuối.

use std::path::Path;

use rusqlite::OpenFlags;

use super::super::super::types::Database;
use super::super::schema::open_database_with_flags;
use super::error::OpenExistingError;

fn open_existing(path: &Path) -> Result<Database, OpenExistingError> {
    let metadata =
        std::fs::symlink_metadata(path).map_err(|source| OpenExistingError::InspectPath {
            path: path.display().to_string(),
            source,
        })?;
    if metadata.file_type().is_symlink() {
        return Err(OpenExistingError::SymlinkedDatabase {
            path: path.display().to_string(),
        });
    }
    if !metadata.is_file() {
        return Err(OpenExistingError::NotRegularFile {
            path: path.display().to_string(),
        });
    }

    let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_NOFOLLOW;
    open_database_with_flags(path, flags).map_err(|source| OpenExistingError::Open {
        path: path.display().to_string(),
        source,
    })
}

impl Database {
    /// Open an existing store for migration and writes, without creating a missing DB.
    /// Mở store có sẵn để migration và ghi, không tạo DB nếu file đang thiếu.
    pub fn open_existing(path: &Path) -> anyhow::Result<Self> {
        Ok(open_existing(path)?)
    }
}
