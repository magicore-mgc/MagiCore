#![cfg(test)]

//! The capability list stays restricted to implemented hardware features.
//! Danh sách chỉ chứa các capability hardware đã triển khai.

use crate::adapter::HardwareAdapter;
use mgc_types::capabilities::Capability;

#[test]
fn capability_manifest_keeps_only_real_providers() {
    let capabilities = HardwareAdapter::CAPABILITIES;
    assert_eq!(capabilities.len(), 3);
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::ProjectDetector))
    );
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::OptimizerProvider))
    );
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::AuditProvider))
    );
}
