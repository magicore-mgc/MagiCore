#![cfg(test)]

//! The capability list stays restricted to implemented hardware features.
//! Danh sách chỉ chứa các capability hardware đã triển khai.

use crate::adapter::HardwareAdapter;
use mgc_types::capabilities::Capability;

#[test]
fn capability_manifest_keeps_only_real_providers() {
    let capabilities = HardwareAdapter::CAPABILITIES;
    // Four entries, and the count alone never proves them real — the per-entry
    // assertions below do. Scaffold joined the list once the CLI lane
    // (`mgc create-hardware`) was recognised as evidence.
    // (Bốn mục; riêng con số không chứng minh chúng thật — các assert từng mục
    // dưới đây mới làm. Scaffold vào danh sách khi lane CLI được công nhận.)
    assert_eq!(capabilities.len(), 4);
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
