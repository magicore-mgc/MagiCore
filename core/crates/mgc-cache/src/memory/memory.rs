//! Shared string interning utility used by cache callers.
//! Tiện ích intern chuỗi dùng chung cho caller của cache.

use std::sync::Arc;

/// Shared string pool to reduce allocations — Pool chuỗi shared giảm allocation.
pub struct StringPool {
    strings: dashmap::DashMap<Arc<str>, ()>,
}

impl StringPool {
    /// Create an empty string pool — Tạo pool chuỗi rỗng.
    pub fn new() -> Self {
        Self {
            strings: dashmap::DashMap::new(),
        }
    }

    /// Intern a string, reusing a stored allocation when available.
    /// Intern chuỗi, tái sử dụng allocation đã lưu nếu có.
    pub fn intern(&self, s: &str) -> Arc<str> {
        if let Some(entry) = self.strings.get(s) {
            return entry.key().clone();
        }

        let shared: Arc<str> = Arc::from(s);
        self.strings.insert(shared.clone(), ());
        shared
    }

    /// Clear all retained strings — Xóa các chuỗi được giữ lại.
    pub fn clear(&self) {
        self.strings.clear();
    }

    /// Return the number of retained strings — Trả số chuỗi đang được giữ.
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Check whether the pool retains no strings — Kiểm tra pool có rỗng.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
    }
}
