#![cfg(test)]

//! Keep only the three implemented capability claims.
//! Chỉ giữ ba capability đã có implementation.

use crate::adapter::CicdAdapter;
use mgc_types::capabilities::Capability;

#[test]
fn capability_manifest_contains_only_detect_scaffold_and_audit() {
    let capabilities = CicdAdapter::CAPABILITIES;
    assert_eq!(capabilities.len(), 3);
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::ProjectDetector))
    );
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::ScaffoldProvider))
    );
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::AuditProvider))
    );
}
