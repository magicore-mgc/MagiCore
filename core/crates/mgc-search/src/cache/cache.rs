//! SearchCache connection owner and stable public API.
//! Owner kết nối SearchCache và API công khai ổn định.

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

use crate::types::{Registry, SearchResult};

use super::error::SearchCacheError;

/// Search cache backed by a local SQLite database.
/// Cache tìm kiếm được lưu trong database SQLite cục bộ.
pub struct SearchCache {
    db: Connection,
}

impl SearchCache {
    /// Open the shared search cache or create it when absent.
    /// Mở cache tìm kiếm dùng chung hoặc tạo mới nếu chưa có.
    ///
    /// Cache location: MGC_DATA_DIR/search_cache.db
    /// Vị trí cache: MGC_DATA_DIR/search_cache.db
    pub fn new() -> Result<Self> {
        let cache_path = Self::cache_path()?;
        Self::new_with_path(&cache_path)
    }

    /// Open a cache at a caller-provided path.
    /// Mở cache tại đường dẫn do caller cung cấp.
    pub fn new_with_path(path: &std::path::Path) -> Result<Self> {
        Self::open(path).map_err(SearchCacheError::into_anyhow)
    }

    fn open(path: &std::path::Path) -> std::result::Result<Self, SearchCacheError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = Connection::open(path)?;
        super::schema::initialize(&db)?;

        Ok(Self { db })
    }

    fn cache_path() -> Result<PathBuf> {
        Ok(mgc_platform::paths::mgc_data_dir().join("search_cache.db"))
    }

    /// Return cached results when the stored row has not expired.
    /// Trả kết quả cache khi bản ghi còn hạn.
    pub fn get(&self, query: &str) -> Result<Option<Vec<SearchResult>>> {
        super::result_cache::get(&self.db, query).map_err(SearchCacheError::into_anyhow)
    }

    /// Replace the cached search results for a query.
    /// Thay thế kết quả cache của một truy vấn.
    pub fn insert(&self, query: &str, results: &[SearchResult]) -> Result<()> {
        super::result_cache::insert(&self.db, query, results).map_err(SearchCacheError::into_anyhow)
    }

    /// Record one package choice for the selected registry.
    /// Ghi nhận một lựa chọn package cho registry đã chọn.
    pub fn track_choice(&self, package_name: &str, registry: Registry) -> Result<()> {
        super::user_choice::track_choice(&self.db, package_name, registry)
            .map_err(SearchCacheError::into_anyhow)
    }

    /// Return the learned registry after the existing three-choice threshold.
    /// Trả registry đã học sau ngưỡng ba lần chọn hiện hành.
    pub fn get_user_choice(&self, package_name: &str) -> Result<Option<Registry>> {
        super::user_choice::get_user_choice(&self.db, package_name)
            .map_err(SearchCacheError::into_anyhow)
    }
}

#[cfg(test)]
#[path = "test/cache.rs"]
mod tests;
