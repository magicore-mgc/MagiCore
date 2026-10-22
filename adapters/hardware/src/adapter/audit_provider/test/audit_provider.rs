#![cfg(test)]

//! The shared audit dispatch capability remains available.
//! Capability dispatch audit dùng chung vẫn khả dụng.

use crate::adapter::HardwareAdapter;
use mgc_types::capabilities::AuditProvider;

#[test]
fn audit_probe_succeeds_for_the_shared_dispatcher() {
    assert!(HardwareAdapter.probe_audit_provider().is_ok());
}
