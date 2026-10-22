//! Verify the five supported contract severity labels.
//! Kiểm tra năm nhãn severity của contract.

use super::super::cyclonedx_severity;
use mgc_types::adapter::VulnerabilitySeverity;

#[test]
fn every_contract_level_maps_to_the_cyclonedx_label() {
    for (level, expected) in [
        (VulnerabilitySeverity::Critical, "critical"),
        (VulnerabilitySeverity::High, "high"),
        (VulnerabilitySeverity::Medium, "medium"),
        (VulnerabilitySeverity::Low, "low"),
        (VulnerabilitySeverity::Info, "info"),
    ] {
        assert_eq!(cyclonedx_severity(&level), expected);
    }
}
