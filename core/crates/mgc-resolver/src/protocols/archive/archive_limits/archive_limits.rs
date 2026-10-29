//! Named limits for untrusted tar expansion on the resolver path.
//! Giới hạn có tên cho việc bung tar không tin cậy trên đường resolver.
//!
//! SEC-15: the resolver extractor enforced per-entry and total size but had no
//! entry-count cap, so a tiny gzip bomb made of millions of zero-length headers
//! could exhaust CPU and disk with metadata work alone.
//! (SEC-15: bộ giải nén resolver có chặn kích thước mỗi entry và tổng nhưng
//! không giới hạn số entry, nên bom gzip bé xíu gồm hàng triệu header rỗng có
//! thể bòn CPU/đĩa chỉ bằng công việc metadata.)

/// Per-entry uncompressed cap (256 MiB).
/// Giới hạn giải nén mỗi entry (256 MiB).
pub(in crate::protocols::archive) const MAX_TAR_ENTRY_UNCOMPRESSED: u64 = 256 * 1024 * 1024;

/// Total uncompressed cap per archive (1 GiB).
/// Giới hạn giải nén tổng mỗi archive (1 GiB).
pub(in crate::protocols::archive) const MAX_TAR_TOTAL_UNCOMPRESSED: u64 = 1024 * 1024 * 1024;

/// Maximum number of tar entries per archive; matches the fetcher limit.
/// Số entry tar tối đa mỗi archive; khớp giới hạn của fetcher.
pub(in crate::protocols::archive) const MAX_TAR_ENTRIES: usize = 16_384;
