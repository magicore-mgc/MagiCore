//! Package-store directory layout and path conventions.
//! Quy ước ánh xạ thư mục package store.

use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Layout {
    root: PathBuf,
}

impl Layout {
    /// Creates a path mapper rooted at the supplied store directory.
    /// Tạo bộ ánh xạ đường dẫn từ thư mục store do caller cung cấp.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Returns the root path without accessing the filesystem.
    /// Trả về root mà không truy cập filesystem.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the content-addressed blob directory path.
    /// Trả về đường dẫn thư mục lưu blob theo nội dung.
    pub fn cas_dir(&self) -> PathBuf {
        self.root.join("cas")
    }

    /// Returns the package tarball cache directory path.
    /// Trả về đường dẫn thư mục cache tarball package.
    pub fn cache_dir(&self) -> PathBuf {
        self.root.join("cache")
    }

    /// Returns the SQLite database path.
    /// Trả về đường dẫn cơ sở dữ liệu SQLite.
    pub fn db_path(&self) -> PathBuf {
        self.root.join("store.db")
    }

    /// Returns the temporary directory path for incomplete downloads.
    /// Trả về đường dẫn thư mục tạm cho download chưa hoàn tất.
    pub fn temp_dir(&self) -> PathBuf {
        self.root.join("tmp")
    }

    /// Returns the log directory path.
    /// Trả về đường dẫn thư mục log.
    pub fn logs_dir(&self) -> PathBuf {
        self.root.join("logs")
    }

    /// Returns the directory path for store locks.
    /// Trả về đường dẫn thư mục lock của store.
    pub fn locks_dir(&self) -> PathBuf {
        self.root.join("locks")
    }

    /// Returns the virtual-store directory path.
    /// Trả về đường dẫn thư mục virtual store.
    pub fn virtual_store_dir(&self) -> PathBuf {
        self.root.join("virtual_store")
    }

    /// Returns the rebuildable MessagePack package-file index path.
    /// Trả về đường dẫn index MessagePack có thể dựng lại từ SQLite.
    pub fn index_msgpack_path(&self) -> PathBuf {
        self.root.join("index.msgpack")
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
