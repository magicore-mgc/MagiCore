//! Wires the shared tar expansion limits.
//! Nối các giới hạn bung tar dùng chung.

mod archive_limits;
pub(in crate::protocols::archive) use archive_limits::{
    MAX_TAR_ENTRIES, MAX_TAR_ENTRY_UNCOMPRESSED, MAX_TAR_TOTAL_UNCOMPRESSED,
};
