//! Named limits for untrusted tar expansion.
//! Giới hạn có tên cho việc bung tar không tin cậy.

pub(in crate::extract) const MAX_ARCHIVE_ENTRIES: usize = 16_384;
pub(in crate::extract) const MAX_ENTRY_BYTES: u64 = 256 * 1024 * 1024;
pub(in crate::extract) const MAX_TOTAL_FILE_BYTES: u64 = 512 * 1024 * 1024;
