//! Validate package names and remove requested manifest entries in order.
//! Kiểm tra tên package và xóa entry trong manifest theo đúng thứ tự request.

use super::super::*;

/// Keep the existing validation, mutation and per-package output order.
/// Giữ thứ tự validate, mutation và output cho từng package hiện tại.
pub(crate) fn remove_requested_dependencies(
    manifest: &mut Manifest,
    packages: &[String],
) -> Result<bool> {
    let mut removed_any = false;
    for package in packages {
        let _ = PackageName::new(package)?;
        if manifest.remove_dep(package) {
            removed_any = true;
            success(&format!("Removed {}", package));
        } else {
            info(&format!("  {} not found in manifest, skipping", package));
        }
    }
    Ok(removed_any)
}
