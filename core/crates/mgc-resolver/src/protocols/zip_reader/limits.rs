//! ZIP resource caps — giới hạn tài nguyên ZIP dùng chung.

/// Per-entry uncompressed cap (256 MiB) — a single zip entry above this is rejected.
/// Giới hạn giải nén mỗi entry 256 MiB — entry vượt ngưỡng bị từ chối.
pub(super) const MAX_ENTRY_UNCOMPRESSED: u64 = 256 * 1024 * 1024;
/// Total uncompressed cap per archive (1 GiB) — giới hạn giải nén tổng mỗi archive.
pub(super) const MAX_TOTAL_UNCOMPRESSED: u64 = 1024 * 1024 * 1024;
/// Maximum number of entries per archive — số entry tối đa mỗi archive.
pub(super) const MAX_ENTRIES: usize = 16_384;
