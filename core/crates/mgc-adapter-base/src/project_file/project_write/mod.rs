//! Unix-only wiring for a dirfd-anchored atomic project write.
//! Nối thao tác ghi atomic neo dirfd chỉ dùng trên Unix.

mod project_write;
pub(super) use self::project_write::atomic_write_project_regular_dirfd;
