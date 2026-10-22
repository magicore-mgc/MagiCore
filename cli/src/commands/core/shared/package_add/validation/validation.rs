//! Validate the shared add request before project mutation starts.
//! Kiểm tra request add dùng chung trước khi bắt đầu mutation project.

use super::super::*;

const MAX_PACKAGES: usize = 50;

/// Preserve the public global/count rejection order and existing errors.
/// Giữ thứ tự từ chối global/count và lỗi hiện có.
pub(crate) fn validate_add_request(package_count: usize, global: bool) -> Result<()> {
    if global {
        return Err(crate::error::global_package_install_unsupported());
    }
    if package_count > MAX_PACKAGES {
        return Err(crate::error::too_many_packages(package_count, "add"));
    }
    Ok(())
}
