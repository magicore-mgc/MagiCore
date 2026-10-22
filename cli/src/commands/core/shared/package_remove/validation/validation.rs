//! Enforce the existing maximum package count for remove.
//! Áp dụng giới hạn số package tối đa hiện có cho remove.

use super::super::*;

const MAX_PACKAGES: usize = 50;

/// Preserve the existing limit and error factory.
/// Giữ nguyên giới hạn và factory lỗi hiện tại.
pub(crate) fn validate_remove_request(package_count: usize) -> Result<()> {
    if package_count > MAX_PACKAGES {
        return Err(crate::error::too_many_packages(package_count, "remove"));
    }
    Ok(())
}
