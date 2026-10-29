#![cfg(test)]

//! Keep only implemented capability claims.
//! Chỉ giữ capability đã có implementation.

use crate::adapter::CicdAdapter;
use mgc_types::capabilities::Capability;

#[test]
fn capability_manifest_contains_only_implemented_capabilities() {
    let capabilities = CicdAdapter::CAPABILITIES;
    // Four claims, each asserted individually below. Optimizer joined once the
    // shared `mgc optimizer` dispatch was recognised as evidence for every core —
    // the same evidence `ai` already declared it with.
    // (Bốn claim, mỗi cái khẳng định riêng bên dưới. Optimizer vào danh sách khi
    // dispatch `mgc optimizer` dùng chung được công nhận — cùng dẫn chứng `ai`
    // đã dùng.)
    assert_eq!(capabilities.len(), 4);
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
    assert!(
        capabilities
            .iter()
            .any(|entry| matches!(entry, Capability::OptimizerProvider))
    );
}
