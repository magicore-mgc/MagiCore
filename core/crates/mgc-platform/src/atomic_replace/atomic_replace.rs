//! Selects the native atomic replacement operation for the current OS.
//! Chọn thao tác thay file atomic native theo OS hiện tại.

use std::path::Path;

use super::AtomicReplaceError;

/// Atomically replace a destination from a same-filesystem staging path.
/// Thay atomic destination từ staging path cùng filesystem.
///
/// Callers must validate staging bytes, own concurrent-writer semantics and sync parents.
/// Caller phải xác minh staging, xử lý writer đồng thời và sync thư mục cha.
pub fn atomic_replace_file(staging: &Path, destination: &Path) -> std::io::Result<()> {
    atomic_replace_file_with_write_through(staging, destination, true).map_err(into_io_error)
}

/// Atomically replace a destination with caller-selected Windows write-through.
/// Thay atomic destination với write-through Windows do caller chọn.
///
/// POSIX rename has no equivalent flag; callers still control file and parent sync.
/// POSIX rename không có cờ tương đương; caller vẫn điều khiển sync file và parent.
pub fn atomic_replace_file_with_write_through(
    staging: &Path,
    destination: &Path,
    write_through: bool,
) -> Result<(), AtomicReplaceError> {
    #[cfg(windows)]
    {
        super::windows_replace(staging, destination, write_through)
    }
    #[cfg(not(windows))]
    {
        let _ = write_through;
        std::fs::rename(staging, destination)
            .map_err(|source| replacement_error(staging, destination, source))
    }
}

pub(super) fn replacement_error(
    staging: &Path,
    destination: &Path,
    source: std::io::Error,
) -> AtomicReplaceError {
    AtomicReplaceError::Replace {
        staging: staging.to_path_buf(),
        destination: destination.to_path_buf(),
        source,
    }
}

fn into_io_error(error: AtomicReplaceError) -> std::io::Error {
    let kind = match &error {
        AtomicReplaceError::Replace { source, .. } => source.kind(),
    };
    std::io::Error::new(kind, error)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
