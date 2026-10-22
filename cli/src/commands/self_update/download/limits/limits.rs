//! Named byte quotas for self-update transfers.
//! Quota byte có tên cho các luồng tải self-update.

/// Maximum release archive accepted (512 MiB — real artifacts are
/// ~50 MiB; anything larger aborts before RAM/disk exhaustion).
/// Quota archive tối đa (chống decompression bomb / nhồi RAM).
pub(in crate::commands) const MAX_ARCHIVE_BYTES: u64 = 512 * 1024 * 1024;

/// Maximum manifest/signature/checksum document (64 KiB — real ones are
/// hundreds of bytes).
/// Quota document nhỏ.
pub(in crate::commands) const MAX_DOC_BYTES: u64 = 64 * 1024;
