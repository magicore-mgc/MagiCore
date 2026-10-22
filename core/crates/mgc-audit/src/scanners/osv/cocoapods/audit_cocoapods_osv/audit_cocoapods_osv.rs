//! bounded Podfile.lock input and honest unsupported-ecosystem reporting.
//! Đọc Podfile.lock và trả trạng thái chưa hỗ trợ advisory một cách trung thực.

use super::super::read_podfile_lock;
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// CocoaPods audit: OSV.dev carries NO CocoaPods ecosystem (verified
/// live 2026-09-10: `{"ecosystem":"pods"}` → 400 "invalid ecosystem").
/// The lockfile parse stays (inventory + skip reasons), but the lane is
/// honestly unsupported — no advisory database exists to query yet.
/// Audit CocoaPods: OSV.dev KHÔNG có ecosystem CocoaPods (đã verify
/// sống: "pods" → 400). Parse lockfile giữ nguyên (kiểm kê + lý do
/// skip), nhưng lane trung thực unsupported — chưa có database
/// advisory để truy vấn.
pub async fn audit_cocoapods_osv(project_root: &Path) -> MgResult<AuditReport> {
    let lock_path = project_root.join("Podfile.lock");
    if !lock_path.is_file() {
        return Ok(AuditReport::unsupported_ecosystem(
            "objc/cocoapods (no Podfile.lock found — run pod install first)",
        ));
    }
    let raw = crate::scanners::read_bounded_project_input(&lock_path, "Podfile.lock")?;
    let (pins, skipped) = read_podfile_lock(&raw)?;
    Ok(super::super::report_cocoapods_inventory(pins, skipped))
}
