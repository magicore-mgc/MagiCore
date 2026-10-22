//! Verify unverified reports never emit findings as evidence.
//! Kiểm tra report chưa xác thực không phát findings thành bằng chứng.

use super::super::to_cyclonedx;
use mgc_types::adapter::AuditReport;

#[test]
fn unsupported_report_has_no_findings_and_carries_unverified_marker() {
    let report = AuditReport::unsupported_ecosystem("unknown project");

    let bom = to_cyclonedx(&report);

    assert!(bom.components.is_empty());
    assert!(bom.vulnerabilities.is_empty());
    assert!(
        bom.metadata.tools.components[0]
            .properties
            .iter()
            .any(|property| property.name == "magicore:status"
                && property.value.contains("UNVERIFIED"))
    );
}
