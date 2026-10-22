//! Policy and freshness diagnostics.
//! Lỗi policy và freshness.

use anyhow::Error;
use anyhow::anyhow;

/// A dependency manifest has an invalid shape for freshness projection.
/// Manifest dependency có cấu trúc không hợp lệ để kiểm freshness.
pub fn freshness_manifest_invalid(file: &str, detail: &str) -> Error {
    anyhow!("cannot check dependency freshness: {file} {detail}")
}

/// A signed policy cache update must not roll back issue time or revocations.
/// Từ chối cập nhật cache policy có ký nếu lùi thời gian hoặc mất revocation.
pub fn policy_manifest_update_rejected(detail: &str) -> Error {
    anyhow!("signed policy manifest update rejected: {detail}")
}

/// A policy cache could not acquire its cross-process writer lock.
/// Không lấy được khóa ghi xuyên tiến trình cho cache policy.
pub fn policy_manifest_cache_lock_failed(detail: &str) -> Error {
    anyhow!("cannot lock signed policy manifest cache: {detail}")
}
