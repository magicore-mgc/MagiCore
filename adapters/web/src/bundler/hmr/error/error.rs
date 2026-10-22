//! Typed wrapper for the original filesystem watcher failure.
//! Wrapper có kiểu cho lỗi filesystem watcher gốc.

/// Notify failure preserved as a source and displayed without added text.
/// Lỗi notify được giữ làm source và hiển thị không thêm text.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(in crate::bundler::hmr) struct HmrWatchError(#[from] notify::Error);

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
