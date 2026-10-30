#![cfg(test)]

//! The capability list stays restricted to implemented hardware features.
//! Danh sách chỉ chứa các capability hardware đã triển khai.

use crate::adapter::HardwareAdapter;
use mgc_types::capabilities::Capability;

#[test]
fn capability_manifest_keeps_only_real_providers() {
    let capabilities = HardwareAdapter::CAPABILITIES;
    // Three entries, and the count alone never proves them real — the per-entry
    // assertions below do. Scaffold joined the list once the CLI lane
    // (`mgc create-hardware`) was recognised as evidence; OptimizerProvider left
    // it (PA-1) because the optimizer/bench template listing is scaffold
    // evidence, not optimizer evidence.
    // (Ba mục; riêng con số không chứng minh chúng thật. Scaffold vào danh sách
    // khi lane CLI được công nhận; OptimizerProvider rời danh sách vì template
    // optimizer/bench là dẫn chứng scaffold, không phải optimizer.)
    assert_eq!(capabilities.len(), 3);
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::ProjectDetector))
    );
    assert!(
        !capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::OptimizerProvider)),
        "optimizer is not claimed: `mgc optimizer` has no hardware-specific lane"
    );
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::AuditProvider))
    );
}
