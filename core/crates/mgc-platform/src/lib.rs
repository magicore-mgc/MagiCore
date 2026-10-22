//! mgc-platform — L0 OS layer (MagiCore)
//! OS layer for standard paths, bounded writes, reflinks, and safe file opening.
//! OS layer cho path chuẩn, ghi có giới hạn, reflink và mở file an toàn.
//!
//! Modules: atomic replace, paths/shell, reflink, fs semaphore, and no-follow file opening.
//! Mô-đun gồm atomic replace, paths/shell, reflink, semaphore ghi và mở file no-follow.

// Unit tests use unwrap for concise fixture setup; production code keeps the workspace lint.
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod atomic_replace;
pub mod fs_semaphore;
pub mod open_regular_file;
pub mod paths;
pub mod reflink;
pub use self::open_regular_file::{OpenRegularFileError, open_regular_file_no_follow};
pub use fs_semaphore::{MAX_CONCURRENT_FS_WRITES, global_fs_write_semaphore};
pub mod prelude {}
