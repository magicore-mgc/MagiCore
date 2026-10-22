//! AuditReport changes pass state only at high severity.
//! AuditReport chỉ đổi trạng thái pass khi severity từ High trở lên.

use super::*;

#[test]
fn test_audit_report_passed() {
    let mut report = AuditReport::new("/tmp/model");
    assert!(report.passed);

    report.add_finding(Finding::medium("test", "warning"));
    assert!(report.passed);

    report.add_finding(Finding::high("test", "error"));
    assert!(!report.passed);
}
