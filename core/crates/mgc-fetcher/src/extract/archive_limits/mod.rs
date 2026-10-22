//! Wires shared archive expansion limits.
//! Nối các giới hạn bung archive dùng chung.

mod archive_limits;
pub(super) use self::archive_limits::{MAX_ARCHIVE_ENTRIES, MAX_ENTRY_BYTES, MAX_TOTAL_FILE_BYTES};
#[cfg(test)]
mod test;
