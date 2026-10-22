//! Unix-only wiring for a no-follow project-root read.
//! Nối thao tác đọc dưới project root chỉ dùng trên Unix.

mod project_read;
pub(super) use self::project_read::read_project_regular_text_dirfd;
