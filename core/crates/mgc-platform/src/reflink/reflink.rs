//! Stable reflink API dispatch for the current target platform.
//! Điều phối API reflink ổn định tới implementation theo OS hiện tại.

use super::ReflinkError;
use std::path::Path;

#[cfg(target_os = "macos")]
use super::macos_clone::reflink_clone as platform_reflink_clone;
#[cfg(all(unix, not(target_os = "macos")))]
use super::unix_clone::reflink_clone as platform_reflink_clone;
#[cfg(not(unix))]
use super::unsupported::reflink_clone as platform_reflink_clone;

/// Clone `source` to `target` as a copy-on-write reflink.
/// Sao chép `source` sang `target` bằng reflink copy-on-write.
///
/// Returns `NotSupported` when the platform/filesystem cannot clone; callers
/// may then select their explicit fallback. Other errors must abort the operation.
/// Trả `NotSupported` nếu OS/filesystem không hỗ trợ; caller chọn fallback tường minh.
/// Lỗi khác phải làm thao tác dừng lại.
pub fn reflink_clone(source: &Path, target: &Path) -> Result<(), ReflinkError> {
    platform_reflink_clone(source, target)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
