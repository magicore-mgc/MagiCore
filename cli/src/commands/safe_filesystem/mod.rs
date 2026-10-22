//! Shared no-follow filesystem checks used by maintenance commands.
//! Kiểm tra filesystem dùng chung, không follow symlink, cho lệnh bảo trì.

mod error;
mod file_copy;
mod measurement;
mod root;

pub(crate) use error::SafeFilesystemError;
pub(crate) use file_copy::{FileCopyError, copy_regular_file_no_follow};
pub(crate) use measurement::logical_payload_bytes;
pub(crate) use root::{canonical_directory, existing_directory_under};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
