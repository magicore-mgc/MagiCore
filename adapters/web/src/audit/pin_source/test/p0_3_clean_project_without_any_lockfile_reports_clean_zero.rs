//! Preserve the existing Web audit workflow contract test.
//! Giữ test hợp đồng workflow audit Web hiện có.

use super::*;

#[test]
fn p0_3_clean_project_without_any_lockfile_reports_clean_zero() {
    // Không lockfile nào → clean 0 (không phải lỗi — không có gì để audit).
    let dir = tempfile::tempdir().unwrap();
    let report = run_audit_blocking(dir.path()).unwrap();
    assert_eq!(report.packages_audited, 0);
    assert_eq!(report.vulnerability_count, 0);
}
