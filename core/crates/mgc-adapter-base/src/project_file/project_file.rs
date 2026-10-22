//! Stable public API and platform dispatcher for project-owned files.
//! Giữ API ổn định và điều phối thao tác theo nền tảng.

use mgc_types::MgError;
use std::path::Path;

/// Read a regular project file without traversing symlinked child components.
/// Đọc file thường dưới project root, không đi qua symlink ở các cấp con.
pub fn read_regular_text(path: &Path, display_name: &str) -> Result<String, MgError> {
    super::regular_file::read_regular_text(path, display_name)
}

/// Read a regular file below a trusted project root without traversing a
/// symlink/reparse point in any project-relative component.
/// Đọc file thường dưới project root tin cậy, không đi qua symlink/reparse.
pub fn read_project_regular_text(
    project_root: &Path,
    relative_path: &Path,
    display_name: &str,
) -> Result<String, MgError> {
    #[cfg(unix)]
    {
        super::project_read::read_project_regular_text_dirfd(
            project_root,
            relative_path,
            display_name,
        )
    }
    #[cfg(not(unix))]
    {
        let path = super::project_path::checked_project_path(project_root, relative_path)?;
        super::regular_file::read_regular_text(&path, display_name)
    }
}

/// Replace an existing regular manifest atomically from a unique staging
/// file on the same filesystem. The destination symlink (if swapped after
/// validation) is replaced as a directory entry, never followed as a target.
/// Thay entry đích bằng staging cùng filesystem; không theo symlink để ghi target.
pub fn atomic_write_regular(path: &Path, bytes: &[u8]) -> Result<(), MgError> {
    super::atomic_file::atomic_write_regular(path, bytes)
}

/// Atomically replace a regular file below a trusted project root. Every
/// existing project-relative parent is checked as a real directory first.
/// Thay file atomically dưới project root; kiểm tra từng thư mục cha hiện hữu.
pub fn atomic_write_project_regular(
    project_root: &Path,
    relative_path: &Path,
    bytes: &[u8],
) -> Result<(), MgError> {
    #[cfg(unix)]
    {
        super::project_write::atomic_write_project_regular_dirfd(project_root, relative_path, bytes)
    }
    #[cfg(not(unix))]
    {
        let path = super::project_path::checked_project_path(project_root, relative_path)?;
        super::atomic_file::atomic_write_regular(&path, bytes)
    }
}

#[cfg(test)]
#[path = "test/project_file.rs"]
mod tests;
