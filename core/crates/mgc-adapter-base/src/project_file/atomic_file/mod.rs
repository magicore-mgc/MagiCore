//! Wiring for atomic replacement of a regular file.
//! Nối thao tác thay thế atomic cho file thường.

mod atomic_file;
pub(super) use self::atomic_file::atomic_write_regular;
