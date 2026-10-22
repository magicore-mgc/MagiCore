//! Self-update compatibility diagnostics delegated to its typed command error owner.
//! Chẩn đoán tương thích self-update chuyển tiếp tới error type của command.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Error;

/// Explicit version failed validation.
/// (Version self-update không hợp lệ.)
pub fn self_update_invalid_version(x: &str) -> Error {
    SelfUpdateError::invalid_version(x)
}
/// Unknown release variant.
/// (Variant release không biết.)
pub fn self_update_unsupported_variant(x: &str) -> Error {
    SelfUpdateError::unsupported_variant(x)
}
/// Host platform outside the release matrix.
/// (Nền tảng máy không trong ma trận release.)
pub fn self_update_unsupported_host(x: &str) -> Error {
    SelfUpdateError::unsupported_host(x)
}
/// Checksum mismatch; abort before extraction or swap.
/// (Checksum lệch; dừng trước giải nén hoặc thay binary.)
pub fn self_update_checksum_mismatch(a: &str, e: &str, x: &str) -> Error {
    SelfUpdateError::checksum_mismatch(a, e, x)
}
/// Archive contains no mgc binary.
/// (Archive không chứa binary mgc.)
pub fn self_update_no_binary(x: &str) -> Error {
    SelfUpdateError::no_binary(x)
}
/// Download failed or exceeded its quota.
/// (Tải thất bại hoặc vượt quota.)
pub fn self_update_download_failed(u: &str, d: &str) -> Error {
    SelfUpdateError::download_failed(u, d)
}
/// New binary failed its launch/version probe; previous version was restored.
/// (Binary mới fail probe; đã phục hồi bản trước.)
pub fn self_update_probe_failed(x: &str) -> Error {
    SelfUpdateError::probe_failed(x)
}
/// Release manifest is missing, malformed or does not bind this artifact.
/// (Manifest release thiếu, sai hoặc không bind artifact.)
pub fn self_update_manifest_invalid(x: &str) -> Error {
    SelfUpdateError::manifest_invalid(x)
}
/// Signature verification failed against configured trust roots.
/// (Chữ ký manifest không verify được.)
pub fn self_update_signature_invalid(x: &str) -> Error {
    SelfUpdateError::signature_invalid(x)
}
/// Unsigned fallback refused by the provenance policy.
/// (Provenance policy từ chối fallback unsigned.)
pub fn self_update_unsigned_refused(x: &str) -> Error {
    SelfUpdateError::unsigned_refused(x)
}
