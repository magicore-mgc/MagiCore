//! Wiring for platform-specific project-root containment checks.
//! Nối bộ kiểm tra containment project theo nền tảng.

mod project_path;
#[cfg(not(unix))]
pub(in crate::project_file) use self::project_path::checked_project_path;
#[cfg(unix)]
pub(super) use self::project_path::project_parent_dir;
