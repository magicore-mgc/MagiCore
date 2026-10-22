//! AuditReport counts critical and high findings.
//! AuditReport đếm finding mức Critical và High.

use super::*;

#[test]
fn test_audit_report_counts() {
    let mut report = AuditReport::new("/tmp/model");
    report.add_finding(Finding::critical("a", "1"));
    report.add_finding(Finding::high("b", "2"));
    report.add_finding(Finding::high("c", "3"));
    report.add_finding(Finding::medium("d", "4"));

    assert_eq!(report.critical_count(), 1);
    assert_eq!(report.high_count(), 2);
}
